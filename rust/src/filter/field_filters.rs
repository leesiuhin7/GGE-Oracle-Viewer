use crate::data::structures::header::Header;

pub(super) trait Predicate<T> {
    fn test(&self, value: T) -> bool;
}

macro_rules! numeric_predicate {
    ($name:ident) => {
        pub(crate) enum $name {
            Eq(Option<i64>),
            Ne(Option<i64>),
            Lt(i64),
            Le(i64),
            Gt(i64),
            Ge(i64),
            Range(i64, i64),
        }

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

macro_rules! string_predicate {
    ($name:ident) => {
        pub(crate) enum $name {
            Eq(Option<String>),
            Ne(Option<String>),
            Prefix(String),
            Suffix(String),
            SubString(String),
        }
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

string_predicate!(BasicNameFilter);
numeric_predicate!(BasicLevelFilter);
numeric_predicate!(BasicLegendaryLevelFilter);
numeric_predicate!(BasicMightFilter);
numeric_predicate!(BasicHonorFilter);
numeric_predicate!(BasicAchievementFilter);
numeric_predicate!(BasicGloryFilter);
numeric_predicate!(BasicRuinsFilter);
numeric_predicate!(AllianceIdFilter);
string_predicate!(AllianceNameFilter);
numeric_predicate!(AllianceRankIdFilter);
numeric_predicate!(AllianceSearchingFilter);
numeric_predicate!(TimerProtectionTimeFilter);
numeric_predicate!(TimerRelocateTimeFilter);

pub(crate) enum LocationFilter {}

pub(crate) enum CoatOfArmsFilter {}

numeric_predicate!(FactionFactionIdFilter);
numeric_predicate!(FactionTitleIdFilter);
numeric_predicate!(FactionSelfProtectionTimeFilter);
numeric_predicate!(FactionGroupProtectionStatusFilter);
numeric_predicate!(FactionGroupProtectionTimeFilter);
numeric_predicate!(FactionMainCampIdFilter);
numeric_predicate!(FactionSpecialCampIdFilter);

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
