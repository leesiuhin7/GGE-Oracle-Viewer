use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{Read, Seek, Write},
};

use crate::{
    query::SnapshotInfo,
    sorting::{
        comparator::{Comparator, ComparisonEngine},
        item::Item,
        parameters::Parameters,
        producer::Producer,
        reader::Reader,
        streams::Streams,
        writer::Writer,
    },
};

pub(super) struct Sorter<R, W, F, P>
where
    R: Read + Seek,
    W: Write,
    F: FnMut(Streams<R, W>) -> Result<Streams<R, W>, ()>,
    P: Producer,
{
    reader: Option<Reader<R>>,
    writer: Option<Writer<W>>,
    swap_fn: F,
    producer: P,
    parameters: Parameters,
}

impl<R, W, F, P> Sorter<R, W, F, P>
where
    R: Read + Seek,
    W: Write,
    F: FnMut(Streams<R, W>) -> Result<Streams<R, W>, ()>,
    P: Producer,
{
    pub(super) fn new(
        reader: R,
        writer: W,
        swap_fn: F,
        producer: P,
        parameters: Parameters,
    ) -> Self {
        Self {
            reader: Some(Reader::new(reader, Vec::new(), parameters.buffer_length)),
            writer: Some(Writer::new(writer)),
            swap_fn,
            producer,
            parameters,
        }
    }

    pub(super) fn into_streams(self) -> Streams<R, W> {
        Streams {
            reader: self.reader.unwrap().into_inner(),
            writer: self.writer.unwrap().into_inner(),
        }
    }

    pub(super) fn sort<T: Write>(
        &mut self,
        iter: impl Iterator<Item = SnapshotInfo>,
        comparators: &[Comparator],
        output: &mut T,
    ) -> Result<u64, ()> {
        // Initialize by writing all SnapshotInfo to writer
        let writer = self.writer.as_mut().unwrap();

        let k = 2.max(
            self.parameters.memory
                / (size_of::<SnapshotInfo>() * (self.parameters.buffer_length + 1)),
        );

        let engine = ComparisonEngine::new(comparators);
        let schema = engine.schema();
        let fields = schema.fields();

        let mut chunk = Vec::with_capacity(k);
        // Break into chunks of k to be sorted
        for info in iter {
            let Some(snapshot) = self.producer.produce(&info, fields) else {
                continue;
            };

            let keys = schema.convert_snapshot(snapshot);
            let item = Item::new(keys, info, 0, &engine);
            chunk.push(item);

            if chunk.len() >= k {
                chunk.sort_unstable();
                let mut session = writer.new_session();
                for Item { info, .. } in &chunk {
                    session.write(info).map_err(|_| ())?;
                }
                chunk.clear();
            }
        }
        // Handle the trailing chunk as well
        if !chunk.is_empty() {
            chunk.sort_unstable();
            let mut session = writer.new_session();
            for Item { info, .. } in &chunk {
                session.write(info).map_err(|_| ())?;
            }
        }

        loop {
            self.swap_streams()?;
            let Ok(completed) = self.k_way_merge(&engine, k) else {
                break;
            };
            if completed {
                break;
            }
        }
        // Prepare for returning results
        self.swap_streams()?;
        let reader = self.reader.as_mut().unwrap().get_mut();
        std::io::copy(reader, output).map_err(|_| ())
    }

    fn k_way_merge(&mut self, engine: &ComparisonEngine, k: usize) -> Result<bool, std::io::Error> {
        // Using unwrap because self.reader and self.writer should never be None
        let reader = self.reader.as_mut().unwrap();
        let writer = self.writer.as_mut().unwrap();
        let mut i = 0;

        let schema = engine.schema();
        let fields = schema.fields();

        // Repeat for all session (but output is not necessarily fully sorted)
        while let Some(mut reader_session) = reader.new_session(i..i + k) {
            // Sort items in the session
            let mut heap = BinaryHeap::with_capacity(k);
            // Initialize heap
            for index in 0..k {
                if let Ok(info) = reader_session.read(index)
                    && let Some(snapshot) = self.producer.produce(&info, fields)
                {
                    let keys = schema.convert_snapshot(snapshot);
                    heap.push(Reverse(Item::new(keys, info, index, engine)));
                }
            }
            // Use heap to sort
            let mut writer_session = writer.new_session();
            while let Some(Reverse(Item { info, index, .. })) = heap.pop() {
                writer_session.write(&info)?;

                if let Ok(new_info) = reader_session.read(index)
                    && let Some(snapshot) = self.producer.produce(&new_info, fields)
                {
                    let keys = schema.convert_snapshot(snapshot);
                    heap.push(Reverse(Item::new(keys, new_info, index, engine)));
                }
            }

            i += k;
        }
        Ok(i <= k) // true if the loop only ran 0 or 1 time
    }

    fn swap_streams(&mut self) -> Result<(), ()> {
        // Using unwrap because self.reader and self.writer should never be None
        let reader = self.reader.take().unwrap();
        let writer = self.writer.take().unwrap();

        let (inner_writer, offsets) = writer.into_inner_and_offsets();
        let streams = (self.swap_fn)(Streams {
            reader: reader.into_inner(),
            writer: inner_writer,
        })?;

        let buffer_len = self.parameters.buffer_length;
        self.reader
            .replace(Reader::new(streams.reader, offsets, buffer_len));
        self.writer.replace(Writer::new(streams.writer));
        Ok(())
    }
}
