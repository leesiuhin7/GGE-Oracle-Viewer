pub mod rle {
    use std::io::{Read, Seek};

    use super::super::primitives::{DecodeVarintError, decode_varint_u64, skip_varint};

    pub struct Run<T> {
        pub value: T,
        pub count: u64,
    }

    #[derive(Debug)]
    pub enum Error {
        Varint,
        Io,
        ReadData,
    }

    impl From<DecodeVarintError> for Error {
        fn from(_: DecodeVarintError) -> Self {
            Error::Varint
        }
    }

    impl From<std::io::Error> for Error {
        fn from(_: std::io::Error) -> Self {
            Error::Io
        }
    }

    impl From<()> for Error {
        fn from((): ()) -> Self {
            Error::ReadData
        }
    }

    pub(in crate::data) fn unpack<T, R: Read + Seek, F>(
        reader: &mut R,
        size: u64,
        read_fn: F,
    ) -> Result<Vec<Run<T>>, Error>
    where
        F: Fn(&mut R) -> Result<T, ()>,
    {
        let end_pos = reader.stream_position()? + size;
        skip_varint(reader)?;
        reader.seek_relative(4)?;

        let mut runs: Vec<Run<T>> = Vec::new();
        while reader.stream_position()? < end_pos {
            let value = read_fn(reader)?;
            let count = decode_varint_u64(reader)?;
            runs.push(Run { value, count });
        }
        Ok(runs)
    }
}

pub mod delta {
    use std::io::{Read, Seek};

    use super::super::primitives::{DecodeVarintError, decode_varint_optional_i64, skip_varint};

    #[derive(Debug)]
    pub enum Error {
        Varint,
        Io,
    }

    impl From<DecodeVarintError> for Error {
        fn from(_: DecodeVarintError) -> Self {
            Error::Varint
        }
    }

    impl From<std::io::Error> for Error {
        fn from(_: std::io::Error) -> Self {
            Error::Io
        }
    }

    pub(in crate::data) fn unpack(
        reader: &mut (impl Read + Seek),
        size: u64,
    ) -> Result<Vec<Option<i64>>, Error> {
        let end_pos = reader.stream_position()? + size;
        skip_varint(reader)?;
        reader.seek_relative(4)?;

        let mut values: Vec<Option<i64>> = Vec::new();
        let mut accumulator = 0i64;
        while reader.stream_position()? < end_pos {
            let optional_delta = decode_varint_optional_i64(reader)?;
            // Push None if delta is None, new accumulator value otherwise
            values.push(optional_delta.map(|delta| {
                accumulator += delta;
                accumulator
            }));
        }
        Ok(values)
    }
}

pub mod delta_rle {
    use std::io::{Read, Seek};

    use super::super::primitives::{
        DecodeVarintError, decode_varint_optional_i64, decode_varint_u64, skip_varint,
    };
    pub struct Run {
        pub delta: Option<i64>,
        pub count: u64,
    }

    #[derive(Debug)]
    pub enum Error {
        Varint,
        Io,
    }

    impl From<DecodeVarintError> for Error {
        fn from(_: DecodeVarintError) -> Self {
            Error::Varint
        }
    }

    impl From<std::io::Error> for Error {
        fn from(_: std::io::Error) -> Self {
            Error::Io
        }
    }

    pub(in crate::data) fn unpack(
        reader: &mut (impl Read + Seek),
        size: u64,
    ) -> Result<Vec<Run>, Error> {
        let end_pos = reader.stream_position()? + size;
        skip_varint(reader)?;
        reader.seek_relative(12)?;

        let mut runs: Vec<Run> = Vec::new();
        while reader.stream_position()? < end_pos {
            let delta = decode_varint_optional_i64(reader)?;
            let count = decode_varint_u64(reader)?;
            runs.push(Run { delta, count });
        }
        Ok(runs)
    }
}

pub mod header {
    use std::io::{Read, Seek};

    use super::super::primitives::{DecodeStringError, decode_optional_string};

    pub struct Header {
        pub id: u32,
        pub server: String,
    }

    pub enum Error {
        Varint,
        Io,
        Server,
    }

    impl From<DecodeStringError> for Error {
        fn from(_: DecodeStringError) -> Self {
            Error::Varint
        }
    }

    impl From<std::io::Error> for Error {
        fn from(_: std::io::Error) -> Self {
            Error::Io
        }
    }

    pub(in crate::data) fn unpack(reader: &mut (impl Read + Seek)) -> Result<Header, Error> {
        let mut buffer = [0u8; 4];
        reader.read_exact(&mut buffer)?;
        let id = u32::from_be_bytes(buffer);
        let server = decode_optional_string(reader)?.ok_or(Error::Server)?;
        Ok(Header { id, server })
    }
}

pub mod locations {
    use std::io::{Read, Seek};

    use super::{super::primitives::decode_optional_varint_array, rle};

    pub struct Location {
        pub kingdom_id: i64,
        pub id: i64,
        pub x: i64,
        pub y: i64,
        #[allow(clippy::struct_field_names)]
        pub location_type: i64,
    }

    fn unpack_locations(array: &[i64]) -> Vec<Location> {
        array
            .as_chunks::<5>()
            .0
            .iter()
            .map(|chunk| Location {
                kingdom_id: chunk[0],
                id: chunk[1],
                x: chunk[2],
                y: chunk[3],
                location_type: chunk[4],
            })
            .collect()
    }

    pub(in crate::data) fn unpack(
        reader: &mut (impl Read + Seek),
        size: u64,
    ) -> Result<Vec<rle::Run<Option<Vec<Location>>>>, rle::Error> {
        Ok(rle::unpack(reader, size, |reader| {
            decode_optional_varint_array(reader).map_err(|_| ())
        })?
        .into_iter()
        .map(|rle::Run { value, count }| rle::Run {
            value: value.map(|array| unpack_locations(&array)),
            count,
        })
        .collect())
    }
}

pub mod coat_of_arms {
    use std::io::{Read, Seek};

    use super::{super::primitives::decode_optional_varint_array, rle};

    pub struct CoatOfArms {
        pub bg_type: i64,
        pub bg_color1: i64,
        pub bg_color2: i64,
        pub symbol_pos_type: i64,
        pub symbol_type1: i64,
        pub symbol_color1: i64,
        pub symbol_type2: i64,
        pub symbol_color2: i64,
    }

    pub enum Error {
        Rle,
        CoatOfArms,
    }

    impl From<rle::Error> for Error {
        fn from(_: rle::Error) -> Self {
            Error::Rle
        }
    }

    impl From<()> for Error {
        fn from(_: ()) -> Self {
            Error::CoatOfArms
        }
    }

    fn unpack_coat_of_arms(array: &[i64]) -> Result<CoatOfArms, ()> {
        if array.len() < 8 {
            Err(())
        } else {
            Ok(CoatOfArms {
                bg_type: array[0],
                bg_color1: array[1],
                bg_color2: array[2],
                symbol_pos_type: array[3],
                symbol_type1: array[4],
                symbol_color1: array[5],
                symbol_type2: array[6],
                symbol_color2: array[7],
            })
        }
    }

    pub(in crate::data) fn unpack(
        reader: &mut (impl Read + Seek),
        size: u64,
    ) -> Result<Vec<rle::Run<Option<CoatOfArms>>>, Error> {
        Ok(rle::unpack(reader, size, |reader| {
            decode_optional_varint_array(reader).map_err(|_| ())
        })?
        .into_iter()
        .map(|rle::Run { value, count }| {
            let coat_of_arms = if let Some(array) = value {
                match unpack_coat_of_arms(&array) {
                    Ok(coat_of_arms) => Some(coat_of_arms),
                    Err(_) => return Err(()),
                }
            } else {
                None
            };
            Ok(rle::Run {
                value: coat_of_arms,
                count,
            })
        })
        .collect::<Result<Vec<_>, _>>()?)
    }
}
