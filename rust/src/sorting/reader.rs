use std::{
    collections::VecDeque,
    io::{Read, Seek, Take},
};

use crate::{query::SnapshotInfo, sorting::codec::decode};

pub(super) struct Session<'a, R: Read + Seek> {
    reader: Take<&'a mut R>,
    pointers: Vec<u64>,
    pointer_max: Vec<u64>,
    buffer: Vec<VecDeque<SnapshotInfo>>,
    buffer_len: usize,
}

impl<'a, R: Read + Seek> Session<'a, R> {
    fn new(reader: Take<&'a mut R>, offsets: Vec<u64>, buffer_len: usize) -> Session<'a, R> {
        let pointer_max = offsets.iter().copied().skip(1).collect();
        let mut pointers = offsets;
        // Using unwrap as offsets should have at least 1 item
        pointers.pop().unwrap();

        let mut buffer = Vec::with_capacity(pointers.len());
        for _ in 0..pointers.len() {
            buffer.push(VecDeque::with_capacity(buffer_len));
        }

        Session {
            reader,
            pointers,
            pointer_max,
            buffer,
            buffer_len,
        }
    }

    pub(super) fn read(&mut self, index: usize) -> Result<SnapshotInfo, ()> {
        if index >= self.pointers.len() {
            return Err(());
        }
        // Prefer using buffer first
        if let Some(snapshot_info) = self.read_buffer(index) {
            return Ok(snapshot_info);
        }
        // Decode only when buffer is empty
        let pointer = self.pointers[index];
        if pointer >= self.pointer_max[index] {
            return Err(());
        }

        self.reader
            .seek(std::io::SeekFrom::Start(pointer))
            .map_err(|_| ())?;
        let snapshot_info = decode(&mut self.reader).map_err(|_| ())?;
        // Only increment pointer if succeeded
        self.pointers[index] += 12;

        // Silencing potential error to allow returning the already decoded SnapshotInfo
        let _ = self.fill_buffer(index);

        Ok(snapshot_info)
    }

    fn read_buffer(&mut self, index: usize) -> Option<SnapshotInfo> {
        let queue = &mut self.buffer[index];
        queue.pop_front()
    }

    fn fill_buffer(&mut self, index: usize) -> Result<(), std::io::Error> {
        let len = self.buffer[index].len();
        // Repeat until buffer is full or the pointer reaches the end
        while self.pointers[index] < self.pointer_max[index] && len < self.buffer_len {
            let snapshot_info = decode(&mut self.reader)?;
            // Only increment pointer if succeeded
            self.pointers[index] += 12;
            // Push to buffer
            self.buffer[index].push_back(snapshot_info);
            self.buffer_len += 1;
        }
        Ok(())
    }
}

pub(super) struct Reader<R: Read + Seek> {
    #[allow(clippy::struct_field_names)]
    reader: R,
    offsets: Vec<u64>,
    buffer_len: usize,
}

impl<R: Read + Seek> Reader<R> {
    pub(super) fn new(reader: R, offsets: Vec<u64>, buffer_len: usize) -> Self {
        Reader {
            reader,
            offsets,
            buffer_len,
        }
    }

    pub(super) fn new_session(&mut self, range: std::ops::Range<usize>) -> Option<Session<'_, R>> {
        let start = range.start;
        let end = self.offsets.len().min(range.end + 1);
        if start < end - 1 {
            // Require length of at least 2 because otherwise size would be 0
            let offsets = self.offsets[start..end].to_vec();
            let size = offsets.last().unwrap() - offsets.first().unwrap();
            let reader = self.reader.by_ref().take(size);
            Some(Session::new(reader, offsets, self.buffer_len))
        } else {
            None
        }
    }

    pub(super) fn into_inner(self) -> R {
        self.reader
    }

    pub(super) fn get_mut(&mut self) -> &mut R {
        // &mut R can be given out as Session can work with a moved pointer
        &mut self.reader
    }
}
