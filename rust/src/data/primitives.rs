use std::{
    io::{Cursor, Read},
    string::FromUtf8Error,
};

#[allow(clippy::cast_possible_wrap)] // Wrapping will never happen
fn decode_zigzag(value: u64) -> i64 {
    ((value >> 1) as i64) ^ -((value & 1) as i64)
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum DecodeVarintError {
    Io(std::io::Error),
    Size(String),
}

impl From<std::io::Error> for DecodeVarintError {
    fn from(value: std::io::Error) -> Self {
        DecodeVarintError::Io(value)
    }
}
impl From<String> for DecodeVarintError {
    fn from(value: String) -> Self {
        DecodeVarintError::Size(value)
    }
}

pub fn decode_varint_u64(reader: &mut impl Read) -> Result<u64, DecodeVarintError> {
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

pub fn skip_varint(reader: &mut impl Read) -> Result<(), std::io::Error> {
    loop {
        let mut buffer = [0u8];
        reader.read_exact(&mut buffer)?;
        if buffer[0] & 0x80 == 0 {
            return Ok(());
        }
    }
}

pub fn decode_varint_i64(reader: &mut impl Read) -> Result<i64, DecodeVarintError> {
    Ok(decode_zigzag(decode_varint_u64(reader)?))
}

pub fn decode_varint_optional_i64(
    reader: &mut impl Read,
) -> Result<Option<i64>, DecodeVarintError> {
    Ok(match decode_varint_u64(reader)? {
        0 => None,
        x => Some(decode_zigzag(x - 1)),
    })
}

pub enum DecodeStringError {
    Varint(DecodeVarintError),
    Io(std::io::Error),
    String(FromUtf8Error),
}

impl From<DecodeVarintError> for DecodeStringError {
    fn from(value: DecodeVarintError) -> Self {
        DecodeStringError::Varint(value)
    }
}

impl From<std::io::Error> for DecodeStringError {
    fn from(value: std::io::Error) -> Self {
        DecodeStringError::Io(value)
    }
}

impl From<FromUtf8Error> for DecodeStringError {
    fn from(value: FromUtf8Error) -> Self {
        DecodeStringError::String(value)
    }
}

pub fn decode_optional_string(reader: &mut impl Read) -> Result<Option<String>, DecodeStringError> {
    Ok(match decode_varint_u64(reader)? {
        0 => None,
        value => {
            let size = value - 1;
            let mut buffer = vec![0u8; size as usize];
            reader.read_exact(&mut buffer)?;
            Some(String::from_utf8(buffer)?)
        }
    })
}

pub fn decode_optional_varint_array(
    reader: &mut impl Read,
) -> Result<Option<Vec<i64>>, DecodeVarintError> {
    match decode_varint_u64(reader)? {
        0 => Ok(None),
        value => {
            let size = value - 1;
            let mut buffer = vec![0u8; size as usize];
            reader.read_exact(&mut buffer)?;
            let mut cursor = Cursor::new(buffer);

            let mut array: Vec<i64> = Vec::new();
            loop {
                let result = decode_varint_i64(&mut cursor);
                match result {
                    Ok(value) => array.push(value),
                    Err(DecodeVarintError::Io(_)) => return Ok(Some(array)),
                    Err(error) => return Err(error),
                }
            }
        }
    }
}
