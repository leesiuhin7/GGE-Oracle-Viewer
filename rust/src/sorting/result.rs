use std::io::{Error, Read, Seek};

use crate::{query::SnapshotInfo, sorting::codec::decode};

pub(crate) struct SortingResult<R: Read + Seek> {
    reader: R,
}

impl<R: Read + Seek> SortingResult<R> {
    pub(crate) fn new(reader: R) -> Self {
        SortingResult { reader }
    }

    pub(crate) fn get(&mut self, range: std::ops::Range<u64>) -> Result<Vec<SnapshotInfo>, Error> {
        let start = range.start;
        self.reader.seek(std::io::SeekFrom::Start(start * 12))?;
        Ok(range
            .into_iter()
            .map_while(|_| decode(&mut self.reader).ok())
            .collect())
    }
}
