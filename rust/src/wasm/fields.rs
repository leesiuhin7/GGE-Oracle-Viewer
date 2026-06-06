use wasm_bindgen::prelude::*;

use crate::data::block::{AllianceField, BasicField, FactionField, Field, TimerField};

#[wasm_bindgen]
pub enum WrapperField {
    Header,
    Timestamp,
    BasicName,
    BasicLevel,
    BasicLegendaryLevel,
    BasicMight,
    BasicHonor,
    BasicAchievement,
    BasicGlory,
    BasicRuins,
    AllianceId,
    AllianceName,
    AllianceRankId,
    AllianceSearching,
    TimerProtectionTime,
    TimerRelocateTime,
    Location,
    CoatOfArms,
    FactionId,
    FactionTitleId,
    FactionSelfProtectionTime,
    FactionGroupProtectionStatus,
    FactionGroupProtectionTime,
    FactionMainCampId,
    FactionSpecialCampId,
}

impl From<Field> for WrapperField {
    fn from(value: Field) -> Self {
        match value {
            Field::Header => WrapperField::Header,
            Field::Timestamp => WrapperField::Timestamp,
            Field::Basic(field) => match field {
                BasicField::Name => WrapperField::BasicName,
                BasicField::Level => WrapperField::BasicLevel,
                BasicField::LegendaryLevel => WrapperField::BasicLegendaryLevel,
                BasicField::Might => WrapperField::BasicMight,
                BasicField::Honor => WrapperField::BasicHonor,
                BasicField::Achievement => WrapperField::BasicAchievement,
                BasicField::Glory => WrapperField::BasicGlory,
                BasicField::Ruins => WrapperField::BasicRuins,
            },
            Field::Alliance(field) => match field {
                AllianceField::Id => WrapperField::AllianceId,
                AllianceField::Name => WrapperField::AllianceName,
                AllianceField::RankId => WrapperField::AllianceRankId,
                AllianceField::Searching => WrapperField::AllianceSearching,
            },
            Field::Timer(field) => match field {
                TimerField::ProtectionTime => WrapperField::TimerProtectionTime,
                TimerField::RelocateTime => WrapperField::TimerRelocateTime,
            },
            Field::Location => WrapperField::Location,
            Field::CoatOfArms => WrapperField::CoatOfArms,
            Field::Faction(field) => match field {
                FactionField::FactionId => WrapperField::FactionId,
                FactionField::TitleId => WrapperField::FactionTitleId,
                FactionField::SelfProtectionTime => WrapperField::FactionSelfProtectionTime,
                FactionField::GroupProtectionStatus => WrapperField::FactionGroupProtectionStatus,
                FactionField::GroupProtectionTime => WrapperField::FactionGroupProtectionTime,
                FactionField::MainCampId => WrapperField::FactionMainCampId,
                FactionField::SpecialCampId => WrapperField::FactionSpecialCampId,
            },
        }
    }
}

impl From<WrapperField> for Field {
    fn from(value: WrapperField) -> Self {
        match value {
            WrapperField::Header => Field::Header,
            WrapperField::Timestamp => Field::Timestamp,
            WrapperField::BasicName => Field::Basic(BasicField::Name),
            WrapperField::BasicLevel => Field::Basic(BasicField::Level),
            WrapperField::BasicLegendaryLevel => Field::Basic(BasicField::LegendaryLevel),
            WrapperField::BasicMight => Field::Basic(BasicField::Might),
            WrapperField::BasicHonor => Field::Basic(BasicField::Honor),
            WrapperField::BasicAchievement => Field::Basic(BasicField::Achievement),
            WrapperField::BasicGlory => Field::Basic(BasicField::Glory),
            WrapperField::BasicRuins => Field::Basic(BasicField::Ruins),
            WrapperField::AllianceId => Field::Alliance(AllianceField::Id),
            WrapperField::AllianceName => Field::Alliance(AllianceField::Name),
            WrapperField::AllianceRankId => Field::Alliance(AllianceField::RankId),
            WrapperField::AllianceSearching => Field::Alliance(AllianceField::Searching),
            WrapperField::TimerProtectionTime => Field::Timer(TimerField::ProtectionTime),
            WrapperField::TimerRelocateTime => Field::Timer(TimerField::RelocateTime),
            WrapperField::Location => Field::Location,
            WrapperField::CoatOfArms => Field::CoatOfArms,
            WrapperField::FactionId => Field::Faction(FactionField::FactionId),
            WrapperField::FactionTitleId => Field::Faction(FactionField::TitleId),
            WrapperField::FactionSelfProtectionTime => {
                Field::Faction(FactionField::SelfProtectionTime)
            }
            WrapperField::FactionGroupProtectionStatus => {
                Field::Faction(FactionField::GroupProtectionStatus)
            }
            WrapperField::FactionGroupProtectionTime => {
                Field::Faction(FactionField::GroupProtectionTime)
            }
            WrapperField::FactionMainCampId => Field::Faction(FactionField::MainCampId),
            WrapperField::FactionSpecialCampId => Field::Faction(FactionField::SpecialCampId),
        }
    }
}
