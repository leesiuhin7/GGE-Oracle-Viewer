use wasm_bindgen::prelude::*;

use crate::data::{
    block::{AllianceField, BasicField, FactionField, Field, TimerField},
    structures::{coat_of_arms, header, locations},
};

pub enum SnapshotData {
    Header(header::Header),
    Timestamp(i64),
    I64(Option<i64>),
    String(Option<String>),
    Locations(Option<Vec<locations::Location>>),
    CoatOfArms(Option<coat_of_arms::CoatOfArms>),
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct HeaderWrapper {
    pub id: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub server: String,
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Header {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<HeaderWrapper>,
    pub present: bool,
}

impl Header {
    fn new(header: header::Header) -> Self {
        let header::Header { id, server } = header;
        Header {
            value: Some(HeaderWrapper { id, server }),
            present: true,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy, Default)]
pub struct I64 {
    pub value: Option<i64>,
    pub present: bool,
}

impl I64 {
    fn new(value: Option<i64>) -> Self {
        I64 {
            value,
            present: true,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct OptionalString {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<String>,
    pub present: bool,
}

impl OptionalString {
    fn new(value: Option<String>) -> Self {
        OptionalString {
            value,
            present: true,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct LocationWrapper {
    pub kingdom_id: i64,
    pub id: i64,
    pub x: i64,
    pub y: i64,
    pub location_type: i64,
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct Locations {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<Vec<LocationWrapper>>,
    pub present: bool,
}

impl Locations {
    fn new(locations: Option<Vec<locations::Location>>) -> Self {
        if let Some(l) = locations {
            Locations {
                value: Some(
                    l.into_iter()
                        .map(
                            |locations::Location {
                                 kingdom_id,
                                 id,
                                 x,
                                 y,
                                 location_type,
                             }| LocationWrapper {
                                kingdom_id,
                                id,
                                x,
                                y,
                                location_type,
                            },
                        )
                        .collect(),
                ),
                present: true,
            }
        } else {
            Locations {
                value: None,
                present: true,
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CoatOfArmsWrapper {
    pub bg_type: i64,
    pub bg_color1: i64,
    pub bg_color2: i64,
    pub symbol_pos_type: i64,
    pub symbol_type1: i64,
    pub symbol_color1: i64,
    pub symbol_type2: i64,
    pub symbol_color2: i64,
}

#[wasm_bindgen]
#[derive(Clone, Default)]
pub struct CoatOfArms {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<CoatOfArmsWrapper>,
    pub present: bool,
}

impl CoatOfArms {
    #[allow(clippy::needless_pass_by_value)]
    fn new(coat_of_arms: Option<coat_of_arms::CoatOfArms>) -> Self {
        if let Some(coat_of_arms::CoatOfArms {
            bg_type,
            bg_color1,
            bg_color2,
            symbol_pos_type,
            symbol_type1,
            symbol_color1,
            symbol_type2,
            symbol_color2,
        }) = coat_of_arms
        {
            CoatOfArms {
                value: Some(CoatOfArmsWrapper {
                    bg_type,
                    bg_color1,
                    bg_color2,
                    symbol_pos_type,
                    symbol_type1,
                    symbol_color1,
                    symbol_type2,
                    symbol_color2,
                }),
                present: true,
            }
        } else {
            CoatOfArms {
                value: None,
                present: true,
            }
        }
    }
}

#[wasm_bindgen]
#[derive(Default)]
pub struct Snapshot {
    #[wasm_bindgen(getter_with_clone)]
    pub header: Header,
    pub timestamp: I64,
    #[wasm_bindgen(getter_with_clone)]
    pub basic_name: OptionalString,
    pub basic_level: I64,
    pub basic_legendary_level: I64,
    pub basic_might: I64,
    pub basic_honor: I64,
    pub basic_achievement: I64,
    pub basic_glory: I64,
    pub basic_ruins: I64,
    pub alliance_id: I64,
    #[wasm_bindgen(getter_with_clone)]
    pub alliance_name: OptionalString,
    pub alliance_rank_id: I64,
    pub alliance_searching: I64,
    pub timer_protection_time: I64,
    pub timer_relocate_time: I64,
    #[wasm_bindgen(getter_with_clone)]
    pub locations: Locations,
    #[wasm_bindgen(getter_with_clone)]
    pub coat_of_arms: CoatOfArms,
    pub faction_id: I64,
    pub faction_title_id: I64,
    pub faction_self_protection_time: I64,
    pub faction_group_protection_status: I64,
    pub faction_group_protection_time: I64,
    pub faction_main_camp_id: I64,
    pub faction_special_camp_id: I64,
}

impl Snapshot {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, field: Field, data: SnapshotData) -> Result<(), ()> {
        match (field, data) {
            (Field::Header, SnapshotData::Header(header)) => self.header = Header::new(header),
            (Field::Timestamp, SnapshotData::Timestamp(timestamp)) => {
                self.timestamp = I64::new(Some(timestamp));
            }
            (Field::Basic(BasicField::Name), SnapshotData::String(name)) => {
                self.basic_name = OptionalString::new(name);
            }
            (Field::Basic(BasicField::Level), SnapshotData::I64(level)) => {
                self.basic_level = I64::new(level);
            }
            (Field::Basic(BasicField::LegendaryLevel), SnapshotData::I64(level)) => {
                self.basic_legendary_level = I64::new(level);
            }
            (Field::Basic(BasicField::Might), SnapshotData::I64(might)) => {
                self.basic_might = I64::new(might);
            }
            (Field::Basic(BasicField::Honor), SnapshotData::I64(honor)) => {
                self.basic_honor = I64::new(honor);
            }
            (Field::Basic(BasicField::Achievement), SnapshotData::I64(achievement)) => {
                self.basic_achievement = I64::new(achievement);
            }
            (Field::Basic(BasicField::Glory), SnapshotData::I64(glory)) => {
                self.basic_glory = I64::new(glory);
            }
            (Field::Basic(BasicField::Ruins), SnapshotData::I64(ruins)) => {
                self.basic_ruins = I64::new(ruins);
            }
            (Field::Alliance(AllianceField::Id), SnapshotData::I64(id)) => {
                self.alliance_id = I64::new(id);
            }
            (Field::Alliance(AllianceField::Name), SnapshotData::String(name)) => {
                self.alliance_name = OptionalString::new(name);
            }
            (Field::Alliance(AllianceField::RankId), SnapshotData::I64(rank_id)) => {
                self.alliance_rank_id = I64::new(rank_id);
            }
            (Field::Alliance(AllianceField::Searching), SnapshotData::I64(searching)) => {
                self.alliance_searching = I64::new(searching);
            }
            (Field::Timer(TimerField::ProtectionTime), SnapshotData::I64(protection_time)) => {
                self.timer_protection_time = I64::new(protection_time);
            }
            (Field::Timer(TimerField::RelocateTime), SnapshotData::I64(relocate_time)) => {
                self.timer_relocate_time = I64::new(relocate_time);
            }
            (Field::Location, SnapshotData::Locations(locations)) => {
                self.locations = Locations::new(locations);
            }
            (Field::CoatOfArms, SnapshotData::CoatOfArms(coat_of_arms)) => {
                self.coat_of_arms = CoatOfArms::new(coat_of_arms);
            }
            (Field::Faction(FactionField::FactionId), SnapshotData::I64(faction_id)) => {
                self.faction_id = I64::new(faction_id);
            }
            (Field::Faction(FactionField::TitleId), SnapshotData::I64(title_id)) => {
                self.faction_title_id = I64::new(title_id);
            }
            (
                Field::Faction(FactionField::SelfProtectionTime),
                SnapshotData::I64(self_protection_time),
            ) => self.faction_self_protection_time = I64::new(self_protection_time),
            (
                Field::Faction(FactionField::GroupProtectionStatus),
                SnapshotData::I64(group_protection_status),
            ) => self.faction_group_protection_status = I64::new(group_protection_status),
            (
                Field::Faction(FactionField::GroupProtectionTime),
                SnapshotData::I64(group_protection_time),
            ) => self.faction_group_protection_time = I64::new(group_protection_time),
            (Field::Faction(FactionField::MainCampId), SnapshotData::I64(main_camp_id)) => {
                self.faction_main_camp_id = I64::new(main_camp_id);
            }
            (Field::Faction(FactionField::SpecialCampId), SnapshotData::I64(special_camp_id)) => {
                self.faction_special_camp_id = I64::new(special_camp_id);
            }

            _ => return Err(()),
        }
        Ok(())
    }
}
