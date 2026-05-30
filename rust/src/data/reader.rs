use std::{
    io::{Read, Seek},
    num::TryFromIntError,
};

use crate::data::{
    block::Block,
    layout::{BlockLayout, Layout},
};

pub enum Error {
    InvalidIndex,
    Io(std::io::Error),
    OutOfRange(TryFromIntError),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<TryFromIntError> for Error {
    fn from(value: TryFromIntError) -> Self {
        Error::OutOfRange(value)
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

pub struct BlockReader<R: Read + Seek> {
    reader: R,
    layout: Layout,
}

impl<R: Read + Seek> BlockReader<R> {
    pub fn new(reader: R, layout: Layout) -> Self {
        BlockReader { reader, layout }
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn get_block(&mut self, index: usize) -> Result<Block<'_>, Error> {
        let layout = self
            .layout
            .block_layouts()
            .get(index)
            .ok_or(Error::InvalidIndex)?;
        build_block(&mut self.reader, layout)
    }

    pub fn blocks(&mut self) -> impl Iterator<Item = Result<Block<'_>, Error>> {
        let reader = &mut self.reader;

        self.layout
            .block_layouts()
            .iter()
            .map(|layout| build_block(reader, layout))
    }
}
