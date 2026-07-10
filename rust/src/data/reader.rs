use std::{
    io::{Read, Seek},
    num::TryFromIntError,
};

use crate::data::{
    block::Block,
    layout::{BlockLayout, Layout},
};

pub(crate) enum Error {
    InvalidIndex,
    Io,
    OutOfRange,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Io
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::OutOfRange
    }
}

fn build_block<'a>(
    reader: &mut (impl Read + Seek),
    layout: &'a BlockLayout,
) -> Result<Block<'a>, Error> {
    let BlockLayout {
        position,
        size,
        offsets,
    } = layout;

    // More efficient seeking for BufReader
    let current_pos = reader.stream_position()?;
    let offset = i64::try_from(*position)? - i64::try_from(current_pos)?;
    reader.seek_relative(offset)?;

    let mut bytes = vec![0u8; *size as usize];
    reader.read_exact(&mut bytes)?;

    Ok(Block::new(bytes.into(), offsets))
}

pub(crate) struct BlockReader<R: Read + Seek> {
    reader: R,
    layout: Layout,
}

impl<R: Read + Seek> BlockReader<R> {
    pub(crate) fn new(reader: R, layout: Layout) -> Self {
        BlockReader { reader, layout }
    }

    pub(crate) fn get_block(&mut self, index: usize) -> Result<Block<'_>, Error> {
        let layout = self
            .layout
            .block_layouts()
            .get(index)
            .ok_or(Error::InvalidIndex)?;
        build_block(&mut self.reader, layout)
    }

    pub(crate) fn blocks(&mut self) -> impl Iterator<Item = Result<Block<'_>, Error>> {
        let reader = &mut self.reader;

        self.layout
            .block_layouts()
            .iter()
            .map(|layout| build_block(reader, layout))
    }
}
