use std::{
    io::{Cursor, Read},
    num::TryFromIntError,
    string::FromUtf8Error,
};

#[allow(clippy::cast_possible_wrap)] // Wrapping will never happen
fn decode_zigzag(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

#[derive(Debug)]
pub enum DecodeVarintError {
    Io,
    Size,
}

impl From<std::io::Error> for DecodeVarintError {
    fn from(_: std::io::Error) -> Self {
        DecodeVarintError::Io
    }
}
impl From<String> for DecodeVarintError {
    fn from(_: String) -> Self {
        DecodeVarintError::Size
    }
}

pub(super) fn decode_varint_u64(reader: &mut impl Read) -> Result<u64, DecodeVarintError> {
    let mut value: u64 = 0;

    for shift in (0..=63).step_by(7) {
        let mut buffer = [0u8];
        reader.read_exact(&mut buffer)?;
        let byte = buffer[0];

        let add_value = (u128::from(byte & 0x7f)) << shift;
        value |= match u64::try_from(add_value) {
            Ok(x) => x,
            Err(_) => break,
        };
        if byte & 0x80 == 0 {
            return Ok(value);
        }
    }
    Err("Varint is too large for u64".to_string())?
}

pub(super) fn skip_varint(reader: &mut impl Read) -> Result<(), std::io::Error> {
    loop {
        let mut buffer = [0u8];
        reader.read_exact(&mut buffer)?;
        if buffer[0] & 0x80 == 0 {
            return Ok(());
        }
    }
}

pub(super) fn decode_varint_i64(reader: &mut impl Read) -> Result<i64, DecodeVarintError> {
    Ok(decode_zigzag(decode_varint_u64(reader)?))
}

pub(super) fn decode_varint_optional_i64(
    reader: &mut impl Read,
) -> Result<Option<i64>, DecodeVarintError> {
    Ok(match decode_varint_u64(reader)? {
        0 => None,
        x => Some(decode_zigzag(x - 1)),
    })
}

pub enum DecodeStringError {
    Varint,
    Io,
    String,
    OutOfRange,
}

impl From<DecodeVarintError> for DecodeStringError {
    fn from(_: DecodeVarintError) -> Self {
        DecodeStringError::Varint
    }
}

impl From<std::io::Error> for DecodeStringError {
    fn from(_: std::io::Error) -> Self {
        DecodeStringError::Io
    }
}

impl From<FromUtf8Error> for DecodeStringError {
    fn from(_: FromUtf8Error) -> Self {
        DecodeStringError::String
    }
}

impl From<TryFromIntError> for DecodeStringError {
    fn from(_: TryFromIntError) -> Self {
        DecodeStringError::OutOfRange
    }
}

pub(super) fn decode_optional_string(
    reader: &mut impl Read,
) -> Result<Option<String>, DecodeStringError> {
    Ok(match decode_varint_u64(reader)? {
        0 => None,
        value => {
            let size = value - 1;
            let mut buffer = vec![0u8; size.try_into()?];
            reader.read_exact(&mut buffer)?;
            Some(String::from_utf8(buffer)?)
        }
    })
}

pub(super) fn decode_optional_varint_array(
    reader: &mut impl Read,
) -> Result<Option<Vec<i64>>, DecodeVarintError> {
    match decode_varint_u64(reader)? {
        0 => Ok(None),
        value => {
            let size = value - 1;
            let mut buffer = vec![0u8; size.try_into().map_err(|_| DecodeVarintError::Size)?];
            reader.read_exact(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);

            let mut array: Vec<i64> = Vec::new();
            loop {
                let result = decode_varint_i64(&mut cursor);
                match result {
                    Ok(value) => array.push(value),
                    Err(DecodeVarintError::Io) => return Ok(Some(array)),
                    Err(error) => return Err(error),
                }
            }
        }
    }
}
