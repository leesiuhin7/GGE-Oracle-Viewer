use wasm_bindgen::prelude::*;

use crate::data::filter::field_filters::{
    AllianceIdFilter, AllianceNameFilter, AllianceRankIdFilter, AllianceSearchingFilter,
    BasicAchievementFilter, BasicGloryFilter, BasicHonorFilter, BasicLegendaryLevelFilter,
    BasicLevelFilter, BasicMightFilter, BasicNameFilter, BasicRuinsFilter, FactionFactionIdFilter,
    FactionGroupProtectionStatusFilter, FactionGroupProtectionTimeFilter, FactionMainCampIdFilter,
    FactionSelfProtectionTimeFilter, FactionSpecialCampIdFilter, FactionTitleIdFilter, FieldFilter,
    HeaderFilter, TimerProtectionTimeFilter, TimerRelocateTimeFilter, TimestampFilter,
};

#[wasm_bindgen]
pub struct Filter(FieldFilter);

impl From<Filter> for FieldFilter {
    fn from(value: Filter) -> Self {
        value.0
    }
}

// Header
#[wasm_bindgen]
impl Filter {
    pub fn header_id_eq(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdEq(value)))
    }

    pub fn header_id_ne(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdNe(value)))
    }

    pub fn header_id_lt(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdLt(value)))
    }

    pub fn header_id_le(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdLe(value)))
    }

    pub fn header_id_gt(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdGt(value)))
    }

    pub fn header_id_ge(value: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdGe(value)))
    }

    pub fn header_id_range(lower: u32, upper: u32) -> Self {
        Self(FieldFilter::Header(HeaderFilter::IdRange(lower, upper)))
    }

    pub fn header_server_eq(value: String) -> Self {
        Self(FieldFilter::Header(HeaderFilter::ServerEq(value)))
    }

    pub fn header_server_ne(value: String) -> Self {
        Self(FieldFilter::Header(HeaderFilter::ServerNe(value)))
    }
}

// Timestamp
#[wasm_bindgen]
impl Filter {
    pub fn timestamp_eq(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Eq(value)))
    }

    pub fn timestamp_ne(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Ne(value)))
    }

    pub fn timestamp_lt(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Lt(value)))
    }

    pub fn timestamp_le(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Le(value)))
    }

    pub fn timestamp_gt(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Gt(value)))
    }

    pub fn timestamp_ge(value: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Ge(value)))
    }

    pub fn timestamp_range(lower: i64, upper: i64) -> Self {
        Self(FieldFilter::Timestamp(TimestampFilter::Range(lower, upper)))
    }
}

// Basic name
#[wasm_bindgen]
impl Filter {
    pub fn basic_name_eq(value: Option<String>) -> Self {
        Self(FieldFilter::BasicName(BasicNameFilter::Eq(value)))
    }

    pub fn basic_name_ne(value: Option<String>) -> Self {
        Self(FieldFilter::BasicName(BasicNameFilter::Ne(value)))
    }

    pub fn basic_name_prefix(value: String) -> Self {
        Self(FieldFilter::BasicName(BasicNameFilter::Prefix(value)))
    }

    pub fn basic_name_suffix(value: String) -> Self {
        Self(FieldFilter::BasicName(BasicNameFilter::Suffix(value)))
    }

    pub fn basic_name_substring(value: String) -> Self {
        Self(FieldFilter::BasicName(BasicNameFilter::SubString(value)))
    }
}

// Alliance name
#[wasm_bindgen]
impl Filter {
    pub fn alliance_name_eq(value: Option<String>) -> Self {
        Self(FieldFilter::AllianceName(AllianceNameFilter::Eq(value)))
    }

    pub fn alliance_name_ne(value: Option<String>) -> Self {
        Self(FieldFilter::AllianceName(AllianceNameFilter::Ne(value)))
    }

    pub fn alliance_name_prefix(value: String) -> Self {
        Self(FieldFilter::AllianceName(AllianceNameFilter::Prefix(value)))
    }

    pub fn alliance_name_suffix(value: String) -> Self {
        Self(FieldFilter::AllianceName(AllianceNameFilter::Suffix(value)))
    }

    pub fn alliance_name_substring(value: String) -> Self {
        Self(FieldFilter::AllianceName(AllianceNameFilter::SubString(
            value,
        )))
    }
}

macro_rules! impl_numeric_filters {
    ($prefix:ident, $variant:ident, $enum_ty:ty) => {
        paste::paste! {
            #[wasm_bindgen]
            impl Filter {
                pub fn [<$prefix _eq>](value: Option<i64>) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Eq(value)))
                }

                pub fn [<$prefix _ne>](value: Option<i64>) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Ne(value)))
                }

                pub fn [<$prefix _lt>](value: i64) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Lt(value)))
                }

                pub fn [<$prefix _le>](value: i64) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Le(value)))
                }

                pub fn [<$prefix _gt>](value: i64) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Gt(value)))
                }

                pub fn [<$prefix _ge>](value: i64) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Ge(value)))
                }

                pub fn [<$prefix _range>](lower: i64, upper: i64) -> Self {
                    Self(FieldFilter::$variant($enum_ty::Range(lower, upper)))
                }
            }
        }
    };
}

// Basic
impl_numeric_filters!(basic_level, BasicLevel, BasicLevelFilter);
impl_numeric_filters!(
    basic_legendary_level,
    BasicLegendaryLevel,
    BasicLegendaryLevelFilter
);
impl_numeric_filters!(basic_might, BasicMight, BasicMightFilter);
impl_numeric_filters!(basic_honor, BasicHonor, BasicHonorFilter);
impl_numeric_filters!(basic_achievement, BasicAchievement, BasicAchievementFilter);
impl_numeric_filters!(basic_glory, BasicGlory, BasicGloryFilter);
impl_numeric_filters!(basic_ruins, BasicRuins, BasicRuinsFilter);

// Alliance
impl_numeric_filters!(alliance_id, AllianceId, AllianceIdFilter);
impl_numeric_filters!(alliance_rank_id, AllianceRankId, AllianceRankIdFilter);
impl_numeric_filters!(
    alliance_searching,
    AllianceSearching,
    AllianceSearchingFilter
);

// Timers
impl_numeric_filters!(
    timer_protection_time,
    TimerProtectionTime,
    TimerProtectionTimeFilter
);
impl_numeric_filters!(
    timer_relocate_time,
    TimerRelocateTime,
    TimerRelocateTimeFilter
);

// Locations

// Coat of arms

// Faction
impl_numeric_filters!(faction_faction_id, FactionFactionId, FactionFactionIdFilter);
impl_numeric_filters!(faction_title_id, FactionTitleId, FactionTitleIdFilter);
impl_numeric_filters!(
    faction_self_protection_time,
    FactionSelfProtectionTime,
    FactionSelfProtectionTimeFilter
);
impl_numeric_filters!(
    faction_group_protection_status,
    FactionGroupProtectionStatus,
    FactionGroupProtectionStatusFilter
);
impl_numeric_filters!(
    faction_group_protection_time,
    FactionGroupProtectionTime,
    FactionGroupProtectionTimeFilter
);
impl_numeric_filters!(
    faction_main_camp_id,
    FactionMainCampId,
    FactionMainCampIdFilter
);
impl_numeric_filters!(
    faction_special_camp_id,
    FactionSpecialCampId,
    FactionSpecialCampIdFilter
);
