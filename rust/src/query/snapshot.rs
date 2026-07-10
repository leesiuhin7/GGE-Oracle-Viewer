use crate::data::{
    block::{AllianceField, BasicField, FactionField, Field, TimerField},
    structures::{coat_of_arms, header, locations},
};

pub(super) enum SnapshotData {
    Header(header::Header),
    Timestamp(i64),
    I64(Option<i64>),
    String(Option<String>),
    Locations(Option<Vec<locations::Location>>),
    CoatOfArms(Option<coat_of_arms::CoatOfArms>),
}

#[derive(Default)]
pub(crate) enum SnapshotField<T> {
    #[default]
    None,
    Some(T),
}

#[derive(Default)]
pub(crate) struct Snapshot {
    pub(crate) header: SnapshotField<header::Header>,
    pub(crate) timestamp: SnapshotField<i64>,
    pub(crate) basic_name: SnapshotField<Option<String>>,
    pub(crate) basic_level: SnapshotField<Option<i64>>,
    pub(crate) basic_legendary_level: SnapshotField<Option<i64>>,
    pub(crate) basic_might: SnapshotField<Option<i64>>,
    pub(crate) basic_honor: SnapshotField<Option<i64>>,
    pub(crate) basic_achievement: SnapshotField<Option<i64>>,
    pub(crate) basic_glory: SnapshotField<Option<i64>>,
    pub(crate) basic_ruins: SnapshotField<Option<i64>>,
    pub(crate) alliance_id: SnapshotField<Option<i64>>,
    pub(crate) alliance_name: SnapshotField<Option<String>>,
    pub(crate) alliance_rank_id: SnapshotField<Option<i64>>,
    pub(crate) alliance_searching: SnapshotField<Option<i64>>,
    pub(crate) timer_protection_time: SnapshotField<Option<i64>>,
    pub(crate) timer_relocate_time: SnapshotField<Option<i64>>,
    pub(crate) locations: SnapshotField<Option<Vec<locations::Location>>>,
    pub(crate) coat_of_arms: SnapshotField<Option<coat_of_arms::CoatOfArms>>,
    pub(crate) faction_id: SnapshotField<Option<i64>>,
    pub(crate) faction_title_id: SnapshotField<Option<i64>>,
    pub(crate) faction_self_protection_time: SnapshotField<Option<i64>>,
    pub(crate) faction_group_protection_status: SnapshotField<Option<i64>>,
    pub(crate) faction_group_protection_time: SnapshotField<Option<i64>>,
    pub(crate) faction_main_camp_id: SnapshotField<Option<i64>>,
    pub(crate) faction_special_camp_id: SnapshotField<Option<i64>>,
}

impl Snapshot {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn set(&mut self, field: &Field, data: SnapshotData) -> Result<(), ()> {
        match (field, data) {
            (Field::Header, SnapshotData::Header(header)) => {
                self.header = SnapshotField::Some(header);
            }
            (Field::Timestamp, SnapshotData::Timestamp(timestamp)) => {
                self.timestamp = SnapshotField::Some(timestamp);
            }
            (Field::Basic(BasicField::Name), SnapshotData::String(name)) => {
                self.basic_name = SnapshotField::Some(name);
            }
            (Field::Basic(BasicField::Level), SnapshotData::I64(level)) => {
                self.basic_level = SnapshotField::Some(level);
            }
            (Field::Basic(BasicField::LegendaryLevel), SnapshotData::I64(level)) => {
                self.basic_legendary_level = SnapshotField::Some(level);
            }
            (Field::Basic(BasicField::Might), SnapshotData::I64(might)) => {
                self.basic_might = SnapshotField::Some(might);
            }
            (Field::Basic(BasicField::Honor), SnapshotData::I64(honor)) => {
                self.basic_honor = SnapshotField::Some(honor);
            }
            (Field::Basic(BasicField::Achievement), SnapshotData::I64(achievement)) => {
                self.basic_achievement = SnapshotField::Some(achievement);
            }
            (Field::Basic(BasicField::Glory), SnapshotData::I64(glory)) => {
                self.basic_glory = SnapshotField::Some(glory);
            }
            (Field::Basic(BasicField::Ruins), SnapshotData::I64(ruins)) => {
                self.basic_ruins = SnapshotField::Some(ruins);
            }
            (Field::Alliance(AllianceField::Id), SnapshotData::I64(id)) => {
                self.alliance_id = SnapshotField::Some(id);
            }
            (Field::Alliance(AllianceField::Name), SnapshotData::String(name)) => {
                self.alliance_name = SnapshotField::Some(name);
            }
            (Field::Alliance(AllianceField::RankId), SnapshotData::I64(rank_id)) => {
                self.alliance_rank_id = SnapshotField::Some(rank_id);
            }
            (Field::Alliance(AllianceField::Searching), SnapshotData::I64(searching)) => {
                self.alliance_searching = SnapshotField::Some(searching);
            }
            (Field::Timer(TimerField::ProtectionTime), SnapshotData::I64(protection_time)) => {
                self.timer_protection_time = SnapshotField::Some(protection_time);
            }
            (Field::Timer(TimerField::RelocateTime), SnapshotData::I64(relocate_time)) => {
                self.timer_relocate_time = SnapshotField::Some(relocate_time);
            }
            (Field::Location, SnapshotData::Locations(locations)) => {
                self.locations = SnapshotField::Some(locations);
            }
            (Field::CoatOfArms, SnapshotData::CoatOfArms(coat_of_arms)) => {
                self.coat_of_arms = SnapshotField::Some(coat_of_arms);
            }
            (Field::Faction(FactionField::FactionId), SnapshotData::I64(faction_id)) => {
                self.faction_id = SnapshotField::Some(faction_id);
            }
            (Field::Faction(FactionField::TitleId), SnapshotData::I64(title_id)) => {
                self.faction_title_id = SnapshotField::Some(title_id);
            }
            (
                Field::Faction(FactionField::SelfProtectionTime),
                SnapshotData::I64(self_protection_time),
            ) => self.faction_self_protection_time = SnapshotField::Some(self_protection_time),
            (
                Field::Faction(FactionField::GroupProtectionStatus),
                SnapshotData::I64(group_protection_status),
            ) => {
                self.faction_group_protection_status = SnapshotField::Some(group_protection_status);
            }
            (
                Field::Faction(FactionField::GroupProtectionTime),
                SnapshotData::I64(group_protection_time),
            ) => self.faction_group_protection_time = SnapshotField::Some(group_protection_time),
            (Field::Faction(FactionField::MainCampId), SnapshotData::I64(main_camp_id)) => {
                self.faction_main_camp_id = SnapshotField::Some(main_camp_id);
            }
            (Field::Faction(FactionField::SpecialCampId), SnapshotData::I64(special_camp_id)) => {
                self.faction_special_camp_id = SnapshotField::Some(special_camp_id);
            }

            _ => return Err(()),
        }
        Ok(())
    }
}
