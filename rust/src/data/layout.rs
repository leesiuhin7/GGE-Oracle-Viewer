use std::{
    io::{ErrorKind, Read, Seek, Write},
    num::TryFromIntError,
};

use crate::data::primitives::{
    DecodeStringError, DecodeVarintError, decode_optional_string, decode_varint_u64, skip_varint,
};

pub(super) type Offset = u32;

pub(super) struct BlockLayout {
    pub(super) position: u64,
    pub(super) size: u32,
    pub(super) offsets: [Offset; 26],
}

impl BlockLayout {
    fn new(position: u64, size: u32, offsets: [Offset; 26]) -> Self {
        BlockLayout {
            position,
            size,
            offsets,
        }
    }
}

pub enum Error {
    Io,
    String,
    Varint,
    OutOfRange,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Error::Io
    }
}

impl From<DecodeStringError> for Error {
    fn from(_: DecodeStringError) -> Self {
        Error::String
    }
}

impl From<DecodeVarintError> for Error {
    fn from(_: DecodeVarintError) -> Self {
        Error::Varint
    }
}

impl From<TryFromIntError> for Error {
    fn from(_: TryFromIntError) -> Self {
        Error::OutOfRange
    }
}

pub struct Layout {
    block_layouts: Vec<BlockLayout>,
}

impl Layout {
    pub fn new() -> Self {
        Layout {
            block_layouts: Vec::new(),
        }
    }

    pub fn from_data(reader: &mut (impl Read + Seek)) -> Result<Self, Error> {
        let mut block_layouts = Vec::new();
        loop {
            let start = reader.stream_position()?;
            let mut offsets: [Offset; 26] = [0; 26];

            // Skip size header
            match skip_varint(reader) {
                Ok(_) => (),
                Err(error) => match error.kind() {
                    // EOF, exit
                    ErrorKind::UnexpectedEof => break,
                    _ => Err(error)?,
                },
            }
            offsets[0] = u32::try_from(reader.stream_position()? - start)?;

            // Header
            reader.read_exact(&mut [0u8; 4])?;
            decode_optional_string(reader)?;
            offsets[1] = u32::try_from(reader.stream_position()? - start)?;

            for offset in offsets.iter_mut().skip(2) {
                let size = decode_varint_u64(reader)?;
                reader.seek_relative(size.cast_signed())?;
                *offset = u32::try_from(reader.stream_position()? - start)?;
            }
            let size = offsets[25];

            block_layouts.push(BlockLayout {
                position: start,
                size,
                offsets,
            });
        }

        Ok(Layout { block_layouts })
    }

    pub fn from_reader(reader: &mut impl Read) -> Result<Self, std::io::Error> {
        let mut block_layouts = Vec::new();
        loop {
            let mut pos_buffer = [0u8; 8];
            match reader.read_exact(&mut pos_buffer) {
                Ok(()) => (),
                Err(error) => match error.kind() {
                    // EOF, exit
                    ErrorKind::UnexpectedEof => break,
                    _ => return Err(error),
                },
            }
            let position = u64::from_le_bytes(pos_buffer);

            let mut offsets: [Offset; 26] = [0; 26];
            for offset in &mut offsets {
                let mut offset_buffer = [0u8; 4];
                reader.read_exact(&mut offset_buffer)?;
                *offset = Offset::from_le_bytes(offset_buffer);
            }
            let size = offsets[25];

            block_layouts.push(BlockLayout::new(position, size, offsets));
        }

        Ok(Layout { block_layouts })
    }

    pub fn to_writer(&self, writer: &mut impl Write) -> Result<(), std::io::Error> {
        for layout in &self.block_layouts {
            writer.write_all(&u64::to_le_bytes(layout.position))?;
            for offset in layout.offsets {
                writer.write_all(&u32::to_le_bytes(offset))?;
            }
        }
        Ok(())
    }

    pub fn block_layouts(&self) -> &Vec<BlockLayout> {
        &self.block_layouts
    }
}
