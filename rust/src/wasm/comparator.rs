use wasm_bindgen::prelude::*;

use crate::sorting::comparator;

#[wasm_bindgen]
pub struct Comparator(comparator::Comparator);

impl From<Comparator> for comparator::Comparator {
    fn from(value: Comparator) -> Self {
        value.0
    }
}

#[wasm_bindgen]
impl Comparator {
    pub fn header_id_asc() -> Comparator {
        Comparator(comparator::Comparator::Header(comparator::Header::IdAsc))
    }

    pub fn header_id_desc() -> Comparator {
        Comparator(comparator::Comparator::Header(comparator::Header::IdDesc))
    }

    pub fn header_server_asc() -> Comparator {
        Comparator(comparator::Comparator::Header(
            comparator::Header::ServerAsc,
        ))
    }

    pub fn header_server_desc() -> Comparator {
        Comparator(comparator::Comparator::Header(
            comparator::Header::ServerDesc,
        ))
    }
}

macro_rules! impl_comparator {
    ($prefix:ident, $variant:ident) => {
        paste::paste! {
            #[wasm_bindgen]
            impl Comparator {
                pub fn [<$prefix _asc>]() -> Comparator {
                    Comparator(comparator::Comparator::$variant(comparator::$variant::Asc))
                }

                pub fn [<$prefix _desc>]() -> Comparator {
                    Comparator(comparator::Comparator::$variant(comparator::$variant::Desc))
                }
            }
        }
    };
}

impl_comparator!(timestamp, Timestamp);

// Basic
impl_comparator!(basic_name, BasicName);
impl_comparator!(basic_level, BasicLevel);
impl_comparator!(basic_legendary_level, BasicLegendaryLevel);
impl_comparator!(basic_might, BasicMight);
impl_comparator!(basic_honor, BasicHonor);
impl_comparator!(basic_achievement, BasicAchievement);
impl_comparator!(basic_glory, BasicGlory);
impl_comparator!(basic_ruins, BasicRuins);

// Alliance
impl_comparator!(alliance_id, AllianceId);
impl_comparator!(alliance_name, AllianceName);
impl_comparator!(alliance_rank_id, AllianceRankId);
impl_comparator!(alliance_searching, AllianceSearching);

// Timers
impl_comparator!(timer_protection_time, TimerProtectionTime);
impl_comparator!(timer_relocate_time, TimerRelocateTime);

// Locations

// Coat of arms

// Faction
impl_comparator!(faction_faction_id, FactionFactionId);
impl_comparator!(faction_title_id, FactionTitleId);
impl_comparator!(faction_self_protection_time, FactionSelfProtectionTime);
impl_comparator!(
    faction_group_protection_status,
    FactionGroupProtectionStatus
);
impl_comparator!(faction_group_protection_time, FactionGroupProtectionTime);
impl_comparator!(faction_main_camp_id, FactionMainCampId);
impl_comparator!(faction_special_camp_id, FactionSpecialCampId);
