use crate::data::structures::header::Header;

pub(super) trait Predicate<T> {
    fn test(&self, value: T) -> bool;
}

macro_rules! impl_numeric_predicate {
    ($name:ident) => {
        impl Predicate<Option<i64>> for $name {
            fn test(&self, value: Option<i64>) -> bool {
                match self {
                    Self::Eq(cmp_value) => value == *cmp_value,
                    Self::Ne(cmp_value) => value != *cmp_value,
                    Self::Lt(cmp_value) => value.is_some_and(|v| v < *cmp_value),
                    Self::Le(cmp_value) => value.is_some_and(|v| v <= *cmp_value),
                    Self::Gt(cmp_value) => value.is_some_and(|v| v > *cmp_value),
                    Self::Ge(cmp_value) => value.is_some_and(|v| v >= *cmp_value),
                    Self::Range(lower, upper) => value.is_some_and(|v| *lower <= v && v < *upper),
                }
            }
        }
    };
}

macro_rules! impl_string_predicate {
    ($name:ident) => {
        impl Predicate<Option<String>> for $name {
            fn test(&self, value: Option<String>) -> bool {
                match self {
                    Self::Eq(string) => value == *string,
                    Self::Ne(string) => value != *string,
                    Self::Prefix(string) => value.is_some_and(|v| v.starts_with(string)),
                    Self::Suffix(string) => value.is_some_and(|v| v.ends_with(string)),
                    Self::SubString(string) => value.is_some_and(|v| v.contains(string)),
                }
            }
        }
    };
}

pub(crate) enum HeaderFilter {
    IdEq(u32),
    IdNe(u32),
    IdLt(u32),
    IdLe(u32),
    IdGt(u32),
    IdGe(u32),
    IdRange(u32, u32),
    ServerEq(String),
    ServerNe(String),
}

impl Predicate<Header> for HeaderFilter {
    fn test(&self, value: Header) -> bool {
        let Header { id, server } = value;
        match self {
            Self::IdEq(cmp_id) => id == *cmp_id,
            Self::IdNe(cmp_id) => id != *cmp_id,
            Self::IdLt(cmp_id) => id < *cmp_id,
            Self::IdLe(cmp_id) => id <= *cmp_id,
            Self::IdGt(cmp_id) => id > *cmp_id,
            Self::IdGe(cmp_id) => id >= *cmp_id,
            Self::IdRange(lower, upper) => *lower <= id && id < *upper,
            Self::ServerEq(cmp_server) => server == *cmp_server,
            Self::ServerNe(cmp_server) => server != *cmp_server,
        }
    }
}

pub(crate) enum TimestampFilter {
    Eq(i64),
    Ne(i64),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}

impl Predicate<i64> for TimestampFilter {
    fn test(&self, value: i64) -> bool {
        match self {
            Self::Eq(cmp_value) => value == *cmp_value,
            Self::Ne(cmp_value) => value != *cmp_value,
            Self::Lt(cmp_value) => value < *cmp_value,
            Self::Le(cmp_value) => value <= *cmp_value,
            Self::Gt(cmp_value) => value > *cmp_value,
            Self::Ge(cmp_value) => value >= *cmp_value,
            Self::Range(lower, upper) => *lower <= value && value < *upper,
        }
    }
}

pub(crate) enum BasicNameFilter {
    Eq(Option<String>),
    Ne(Option<String>),
    Prefix(String),
    Suffix(String),
    SubString(String),
}
impl_string_predicate!(BasicNameFilter);

pub(crate) enum BasicLevelFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicLevelFilter);

pub(crate) enum BasicLegendaryLevelFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicLegendaryLevelFilter);

pub(crate) enum BasicMightFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicMightFilter);

pub(crate) enum BasicHonorFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicHonorFilter);

pub(crate) enum BasicAchievementFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicAchievementFilter);

pub(crate) enum BasicGloryFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicGloryFilter);

pub(crate) enum BasicRuinsFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(BasicRuinsFilter);

pub(crate) enum AllianceIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(AllianceIdFilter);

pub(crate) enum AllianceNameFilter {
    Eq(Option<String>),
    Ne(Option<String>),
    Prefix(String),
    Suffix(String),
    SubString(String),
}
impl_string_predicate!(AllianceNameFilter);

pub(crate) enum AllianceRankIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(AllianceRankIdFilter);

pub(crate) enum AllianceSearchingFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(AllianceSearchingFilter);

pub(crate) enum TimerProtectionTimeFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(TimerProtectionTimeFilter);

pub(crate) enum TimerRelocateTimeFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(TimerRelocateTimeFilter);

pub(crate) enum LocationFilter {}

pub(crate) enum CoatOfArmsFilter {}

pub(crate) enum FactionFactionIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionFactionIdFilter);

pub(crate) enum FactionTitleIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionTitleIdFilter);

pub(crate) enum FactionSelfProtectionTimeFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionSelfProtectionTimeFilter);

pub(crate) enum FactionGroupProtectionStatusFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionGroupProtectionStatusFilter);

pub(crate) enum FactionGroupProtectionTimeFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionGroupProtectionTimeFilter);

pub(crate) enum FactionMainCampIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionMainCampIdFilter);

pub(crate) enum FactionSpecialCampIdFilter {
    Eq(Option<i64>),
    Ne(Option<i64>),
    Lt(i64),
    Le(i64),
    Gt(i64),
    Ge(i64),
    Range(i64, i64),
}
impl_numeric_predicate!(FactionSpecialCampIdFilter);

pub(crate) enum FieldFilter {
    // Header
    Header(HeaderFilter),
    // Timestamp
    Timestamp(TimestampFilter),
    // Basic
    BasicName(BasicNameFilter),
    BasicLevel(BasicLevelFilter),
    BasicLegendaryLevel(BasicLegendaryLevelFilter),
    BasicMight(BasicMightFilter),
    BasicHonor(BasicHonorFilter),
    BasicAchievement(BasicAchievementFilter),
    BasicGlory(BasicGloryFilter),
    BasicRuins(BasicRuinsFilter),
    // Alliance
    AllianceId(AllianceIdFilter),
    AllianceName(AllianceNameFilter),
    AllianceRankId(AllianceRankIdFilter),
    AllianceSearching(AllianceSearchingFilter),
    // Timers
    TimerProtectionTime(TimerProtectionTimeFilter),
    TimerRelocateTime(TimerRelocateTimeFilter),
    // Locations
    #[allow(dead_code)] // Not yet implemented
    Location(LocationFilter),
    // Coat of arms
    #[allow(dead_code)] // Not yet implemented
    CoatOfArms(CoatOfArmsFilter),
    // Faction
    FactionFactionId(FactionFactionIdFilter),
    FactionTitleId(FactionTitleIdFilter),
    FactionSelfProtectionTime(FactionSelfProtectionTimeFilter),
    FactionGroupProtectionStatus(FactionGroupProtectionStatusFilter),
    FactionGroupProtectionTime(FactionGroupProtectionTimeFilter),
    FactionMainCampId(FactionMainCampIdFilter),
    FactionSpecialCampId(FactionSpecialCampIdFilter),
}
