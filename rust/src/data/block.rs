use std::io::Cursor;

use crate::data::layout::Offset;
use crate::data::primitives::decode_varint_optional_i64;

use super::primitives::decode_optional_string;
use super::structures::{coat_of_arms, delta, delta_rle, header, locations, rle};

pub enum Data {
    Header(header::Header),
    Deltas(Vec<Option<i64>>),
    Timestamps(Vec<i64>),
    RleDelta(Vec<delta_rle::Run>),
    RleI64(Vec<rle::Run<Option<i64>>>),
    RleString(Vec<rle::Run<Option<String>>>),
    RleLocations(Vec<rle::Run<Option<Vec<locations::Location>>>>),
    RleCoatOfArms(Vec<rle::Run<Option<coat_of_arms::CoatOfArms>>>),
}

pub enum Error {
    FieldOutOfRange,
    Header(header::Error),
    Rle(rle::Error),
    Delta(delta::Error),
    DeltaRle(delta_rle::Error),
    CoatOfArms(coat_of_arms::Error),
    BadTimestamp,
}

impl From<header::Error> for Error {
    fn from(value: header::Error) -> Self {
        Error::Header(value)
    }
}

impl From<rle::Error> for Error {
    fn from(value: rle::Error) -> Self {
        Error::Rle(value)
    }
}

impl From<delta::Error> for Error {
    fn from(value: delta::Error) -> Self {
        Error::Delta(value)
    }
}

impl From<delta_rle::Error> for Error {
    fn from(value: delta_rle::Error) -> Self {
        Error::DeltaRle(value)
    }
}
impl From<coat_of_arms::Error> for Error {
    fn from(value: coat_of_arms::Error) -> Self {
        Error::CoatOfArms(value)
    }
}

pub enum BasicField {
    Name = 2,
    Level = 3,
    LegendaryLevel = 4,
    Might = 5,
    Honor = 6,
    Achievement = 7,
    Glory = 8,
    Ruins = 9,
}

pub enum AllianceField {
    Id = 10,
    Name = 11,
    RankId = 12,
    Searching = 13,
}

pub enum TimerField {
    ProtectionTime = 14,
    RelocateTime = 15,
}

pub enum FactionField {
    FactionId = 18,
    TitleId = 19,
    SelfProtectionTime = 20,
    GroupProtectionStatus = 21,
    GroupProtectionTime = 22,
    MainCampId = 23,
    SpecialCampId = 24,
}

#[repr(usize)]
pub enum Field {
    Header = 0,
    Timestamp = 1,
    Basic(BasicField),
    Alliance(AllianceField),
    Timer(TimerField),
    Location = 16,
    CoatOfArms = 17,
    Faction(FactionField),
}

impl Field {
    fn as_usize(&self) -> usize {
        match self {
            Field::Header => 0,
            Field::Timestamp => 1,
            Field::Basic(field) => match field {
                BasicField::Name => 2,
                BasicField::Level => 3,
                BasicField::LegendaryLevel => 4,
                BasicField::Might => 5,
                BasicField::Honor => 6,
                BasicField::Achievement => 7,
                BasicField::Glory => 8,
                BasicField::Ruins => 9,
            },
            Field::Alliance(field) => match field {
                AllianceField::Id => 10,
                AllianceField::Name => 11,
                AllianceField::RankId => 12,
                AllianceField::Searching => 13,
            },
            Field::Timer(field) => match field {
                TimerField::ProtectionTime => 14,
                TimerField::RelocateTime => 15,
            },
            Field::Location => 16,
            Field::CoatOfArms => 17,
            Field::Faction(field) => match field {
                FactionField::FactionId => 18,
                FactionField::TitleId => 19,
                FactionField::SelfProtectionTime => 20,
                FactionField::GroupProtectionStatus => 21,
                FactionField::GroupProtectionTime => 22,
                FactionField::MainCampId => 23,
                FactionField::SpecialCampId => 24,
            },
        }
    }
}

pub struct Block<'a> {
    cursor: Cursor<Box<[u8]>>,
    offsets: &'a [Offset],
}

impl<'a> Block<'a> {
    pub fn new(bytes: Box<[u8]>, offsets: &'a [u32]) -> Self {
        Block {
            cursor: Cursor::new(bytes),
            offsets,
        }
    }

    pub fn read_field(&mut self, field: &Field) -> Result<Data, Error> {
        let offset = self.offsets[field.as_usize()];
        let size = u64::from(self.offsets[field.as_usize() + 1] - offset);

        self.cursor.set_position(u64::from(offset));

        match field {
            Field::Header => Ok(Data::Header(header::unpack(&mut self.cursor)?)),
            Field::Timestamp => {
                let timestamps = delta::unpack(&mut self.cursor, size)?
                    .into_iter()
                    .collect::<Option<Vec<i64>>>()
                    .ok_or(Error::BadTimestamp)?;
                Ok(Data::Timestamps(timestamps))
            }
            Field::Basic(BasicField::Name) | Field::Alliance(AllianceField::Name) => {
                let runs = rle::unpack(&mut self.cursor, size, |reader| {
                    decode_optional_string(reader).map_err(|_| ())
                })?;
                Ok(Data::RleString(runs))
            }
            Field::Basic(BasicField::Might | BasicField::Glory)
            | Field::Timer(TimerField::ProtectionTime | TimerField::RelocateTime)
            | Field::Faction(
                FactionField::SelfProtectionTime | FactionField::GroupProtectionTime,
            ) => Ok(Data::RleDelta(delta_rle::unpack(&mut self.cursor, size)?)),
            Field::Location => Ok(Data::RleLocations(locations::unpack(
                &mut self.cursor,
                size,
            )?)),
            Field::CoatOfArms => Ok(Data::RleCoatOfArms(coat_of_arms::unpack(
                &mut self.cursor,
                size,
            )?)),
            Field::Basic(
                BasicField::Level
                | BasicField::LegendaryLevel
                | BasicField::Honor
                | BasicField::Achievement
                | BasicField::Ruins,
            )
            | Field::Alliance(
                AllianceField::Id | AllianceField::RankId | AllianceField::Searching,
            )
            | Field::Faction(
                FactionField::FactionId
                | FactionField::TitleId
                | FactionField::GroupProtectionStatus
                | FactionField::MainCampId
                | FactionField::SpecialCampId,
            ) => {
                let runs = rle::unpack(&mut self.cursor, size, |reader| {
                    decode_varint_optional_i64(reader).map_err(|_| ())
                })?;
                Ok(Data::RleI64(runs))
            }
        }
    }
}
