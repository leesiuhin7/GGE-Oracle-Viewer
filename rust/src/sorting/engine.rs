use std::io::{Read, Seek, Write};

use crate::{
    query::SnapshotInfo,
    sorting::{Parameters, Streams, comparator::Comparator, producer::Producer, sorter::Sorter},
};

pub(crate) struct Engine<R, W, F>
where
    R: Read + Seek,
    W: Write,
    F: FnMut(Streams<R, W>) -> Result<Streams<R, W>, ()>,
{
    reader: Option<R>,
    writer: Option<W>,
    swap_fn: F,
    parameters: Parameters,
}

impl<R, W, F> Engine<R, W, F>
where
    R: Read + Seek,
    W: Write,
    F: FnMut(Streams<R, W>) -> Result<Streams<R, W>, ()>,
{
    pub(crate) fn new(reader: R, writer: W, swap_fn: F, parameters: Parameters) -> Self {
        Engine {
            reader: Some(reader),
            writer: Some(writer),
            swap_fn,
            parameters,
        }
    }

    pub(crate) fn sort<I, P, T>(
        &mut self,
        iter: I,
        producer: P,
        comparators: &[Comparator],
        output: &mut T,
    ) -> Result<u64, ()>
    where
        I: Iterator<Item = SnapshotInfo>,
        P: Producer,
        T: Write,
    {
        let reader = self.reader.take().unwrap();
        let writer = self.writer.take().unwrap();
        let mut sorter = Sorter::new(
            reader,
            writer,
            &mut self.swap_fn,
            producer,
            self.parameters.clone(),
        );
        let result = sorter.sort(iter, comparators, output);

        // Take reader and writer back
        let Streams { reader, writer } = sorter.into_streams();
        self.reader.replace(reader);
        self.writer.replace(writer);

        result
    }
}
