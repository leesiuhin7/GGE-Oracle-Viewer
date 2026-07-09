use std::io::Write;

use crate::{query::SnapshotInfo, sorting::codec::encode};

pub(super) struct Session<'a, W: Write> {
    writer: &'a mut W,
    offsets: &'a mut Vec<u64>,
    pointer: u64,
}

impl<'a, W: Write> Session<'a, W> {
    fn new(writer: &'a mut W, offsets: &'a mut Vec<u64>) -> Self {
        let pointer = *offsets.last().unwrap();
        Session {
            writer,
            offsets,
            pointer,
        }
    }

    pub(super) fn write(&mut self, snapshot_info: &SnapshotInfo) -> Result<(), std::io::Error> {
        encode(self.writer, snapshot_info)?;
        self.pointer += 12;
        Ok(())
    }
}

impl<W: Write> Drop for Session<'_, W> {
    fn drop(&mut self) {
        self.offsets.push(self.pointer);
    }
}

pub(super) struct Writer<W: Write> {
    writer: W,
    offsets: Vec<u64>,
}

impl<W: Write> Writer<W> {
    pub(super) fn new(writer: W) -> Self {
        Writer {
            writer,
            offsets: vec![0],
        }
    }

    pub(super) fn new_session(&mut self) -> Session<'_, W> {
        Session::new(&mut self.writer, &mut self.offsets)
    }

    pub(super) fn into_inner(self) -> W {
        self.writer
    }

    pub(super) fn into_inner_and_offsets(self) -> (W, Vec<u64>) {
        (self.writer, self.offsets)
    }
}
