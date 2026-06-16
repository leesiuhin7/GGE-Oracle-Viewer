use wasm_bindgen::prelude::*;

use crate::{
    data::structures::{coat_of_arms, header, locations},
    query,
};

#[wasm_bindgen]
#[derive(Clone)]
pub struct HeaderWrapper {
    pub id: u32,
    #[wasm_bindgen(getter_with_clone)]
    pub server: String,
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Header {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<HeaderWrapper>,
    pub present: bool,
}

impl From<query::SnapshotField<header::Header>> for Header {
    fn from(value: query::SnapshotField<header::Header>) -> Self {
        match value {
            query::SnapshotField::None => Header {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(header::Header { id, server }) => Header {
                value: Some(HeaderWrapper { id, server }),
                present: true,
            },
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct I64 {
    pub value: Option<i64>,
    pub present: bool,
}

impl From<query::SnapshotField<Option<i64>>> for I64 {
    fn from(value: query::SnapshotField<Option<i64>>) -> Self {
        match value {
            query::SnapshotField::None => I64 {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(v) => I64 {
                value: v,
                present: true,
            },
        }
    }
}

impl From<query::SnapshotField<i64>> for I64 {
    fn from(value: query::SnapshotField<i64>) -> Self {
        match value {
            query::SnapshotField::None => I64 {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(v) => I64 {
                value: Some(v),
                present: true,
            },
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct OptionalString {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<String>,
    pub present: bool,
}

impl From<query::SnapshotField<Option<String>>> for OptionalString {
    fn from(value: query::SnapshotField<Option<String>>) -> Self {
        match value {
            query::SnapshotField::None => OptionalString {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(string) => OptionalString {
                value: string,
                present: true,
            },
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
#[derive(Clone)]
pub struct Locations {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<Vec<LocationWrapper>>,
    pub present: bool,
}

impl From<query::SnapshotField<Option<Vec<locations::Location>>>> for Locations {
    fn from(value: query::SnapshotField<Option<Vec<locations::Location>>>) -> Self {
        match value {
            query::SnapshotField::None => Locations {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(locations) => {
                let wrappers = locations.map(|l| {
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
                        .collect::<Vec<_>>()
                });

                Locations {
                    value: wrappers,
                    present: true,
                }
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
#[derive(Clone)]
pub struct CoatOfArms {
    #[wasm_bindgen(getter_with_clone)]
    pub value: Option<CoatOfArmsWrapper>,
    pub present: bool,
}

impl From<query::SnapshotField<Option<coat_of_arms::CoatOfArms>>> for CoatOfArms {
    fn from(value: query::SnapshotField<Option<coat_of_arms::CoatOfArms>>) -> Self {
        match value {
            query::SnapshotField::None => CoatOfArms {
                value: None,
                present: false,
            },
            query::SnapshotField::Some(coat_of_arms) => {
                let wrapper = coat_of_arms.map(
                    |coat_of_arms::CoatOfArms {
                         bg_type,
                         bg_color1,
                         bg_color2,
                         symbol_pos_type,
                         symbol_type1,
                         symbol_color1,
                         symbol_type2,
                         symbol_color2,
                     }| CoatOfArmsWrapper {
                        bg_type,
                        bg_color1,
                        bg_color2,
                        symbol_pos_type,
                        symbol_type1,
                        symbol_color1,
                        symbol_type2,
                        symbol_color2,
                    },
                );
                CoatOfArms {
                    value: wrapper,
                    present: true,
                }
            }
        }
    }
}

#[wasm_bindgen]
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

impl From<query::Snapshot> for Snapshot {
    fn from(value: query::Snapshot) -> Self {
        let query::Snapshot {
            header,
            timestamp,
            basic_name,
            basic_level,
            basic_legendary_level,
            basic_might,
            basic_honor,
            basic_achievement,
            basic_glory,
            basic_ruins,
            alliance_id,
            alliance_name,
            alliance_rank_id,
            alliance_searching,
            timer_protection_time,
            timer_relocate_time,
            locations,
            coat_of_arms,
            faction_id,
            faction_title_id,
            faction_self_protection_time,
            faction_group_protection_status,
            faction_group_protection_time,
            faction_main_camp_id,
            faction_special_camp_id,
        } = value;

        Snapshot {
            header: header.into(),
            timestamp: timestamp.into(),
            basic_name: basic_name.into(),
            basic_level: basic_level.into(),
            basic_legendary_level: basic_legendary_level.into(),
            basic_might: basic_might.into(),
            basic_honor: basic_honor.into(),
            basic_achievement: basic_achievement.into(),
            basic_glory: basic_glory.into(),
            basic_ruins: basic_ruins.into(),
            alliance_id: alliance_id.into(),
            alliance_name: alliance_name.into(),
            alliance_rank_id: alliance_rank_id.into(),
            alliance_searching: alliance_searching.into(),
            timer_protection_time: timer_protection_time.into(),
            timer_relocate_time: timer_relocate_time.into(),
            locations: locations.into(),
            coat_of_arms: coat_of_arms.into(),
            faction_id: faction_id.into(),
            faction_title_id: faction_title_id.into(),
            faction_self_protection_time: faction_self_protection_time.into(),
            faction_group_protection_status: faction_group_protection_status.into(),
            faction_group_protection_time: faction_group_protection_time.into(),
            faction_main_camp_id: faction_main_camp_id.into(),
            faction_special_camp_id: faction_special_camp_id.into(),
        }
    }
}
