use std::cmp::Ordering;

use crate::{
    data::{
        block::{AllianceField, BasicField, FactionField, Field, TimerField},
        structures::header,
    },
    sorting::{data::Data, schema::Schema},
};

trait Compare<T> {
    fn compare(&self, value: &T, other: &T) -> Ordering;
}

fn reverse_ordering(ordering: Ordering) -> Ordering {
    match ordering {
        Ordering::Equal => Ordering::Equal,
        Ordering::Less => Ordering::Greater,
        Ordering::Greater => Ordering::Less,
    }
}

pub(crate) enum Header {
    IdAsc,
    IdDesc,
    ServerAsc,
    ServerDesc,
}

impl Compare<header::Header> for Header {
    fn compare(&self, value: &header::Header, other: &header::Header) -> Ordering {
        let ordering = match self {
            Header::IdAsc | Header::IdDesc => value.id.cmp(&other.id),
            Header::ServerAsc | Header::ServerDesc => value.server.cmp(&other.server),
        };
        match self {
            Header::IdAsc | Header::ServerAsc => ordering,
            Header::IdDesc | Header::ServerDesc => reverse_ordering(ordering),
        }
    }
}

pub(crate) enum Timestamp {
    Asc,
    Desc,
}

impl Compare<i64> for Timestamp {
    fn compare(&self, value: &i64, other: &i64) -> Ordering {
        let ordering = value.cmp(other);
        match self {
            Timestamp::Asc => ordering,
            Timestamp::Desc => reverse_ordering(ordering),
        }
    }
}

macro_rules! make_numeric_comparator {
    ($name:ident) => {
        pub(crate) enum $name {
            Asc,
            Desc,
        }

        impl Compare<Option<i64>> for $name {
            fn compare(&self, value: &Option<i64>, other: &Option<i64>) -> Ordering {
                let ordering = match (value, other) {
                    (Some(a), Some(b)) => a.cmp(b),
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                };
                match self {
                    $name::Asc => ordering,
                    $name::Desc => reverse_ordering(ordering),
                }
            }
        }
    };
}

macro_rules! make_string_comparator {
    ($name:ident) => {
        pub(crate) enum $name {
            Asc,
            Desc,
        }

        impl Compare<Option<String>> for $name {
            fn compare(&self, value: &Option<String>, other: &Option<String>) -> Ordering {
                let ordering = match (value, other) {
                    (Some(a), Some(b)) => a.cmp(b),
                    (None, None) => Ordering::Equal,
                    (None, _) => Ordering::Greater,
                    (_, None) => Ordering::Less,
                };
                match self {
                    $name::Asc => ordering,
                    $name::Desc => reverse_ordering(ordering),
                }
            }
        }
    };
}

// Basic
make_string_comparator!(BasicName);
make_numeric_comparator!(BasicLevel);
make_numeric_comparator!(BasicLegendaryLevel);
make_numeric_comparator!(BasicMight);
make_numeric_comparator!(BasicHonor);
make_numeric_comparator!(BasicAchievement);
make_numeric_comparator!(BasicGlory);
make_numeric_comparator!(BasicRuins);

// Alliance
make_numeric_comparator!(AllianceId);
make_string_comparator!(AllianceName);
make_numeric_comparator!(AllianceRankId);
make_numeric_comparator!(AllianceSearching);

// Timers
make_numeric_comparator!(TimerProtectionTime);
make_numeric_comparator!(TimerRelocateTime);

// Locations

// Coat of arms

// Faction
make_numeric_comparator!(FactionFactionId);
make_numeric_comparator!(FactionTitleId);
make_numeric_comparator!(FactionSelfProtectionTime);
make_numeric_comparator!(FactionGroupProtectionStatus);
make_numeric_comparator!(FactionGroupProtectionTime);
make_numeric_comparator!(FactionMainCampId);
make_numeric_comparator!(FactionSpecialCampId);

pub(crate) enum Comparator {
    Header(Header),
    Timestamp(Timestamp),
    BasicName(BasicName),
    BasicLevel(BasicLevel),
    BasicLegendaryLevel(BasicLegendaryLevel),
    BasicMight(BasicMight),
    BasicHonor(BasicHonor),
    BasicAchievement(BasicAchievement),
    BasicGlory(BasicGlory),
    BasicRuins(BasicRuins),
    AllianceId(AllianceId),
    AllianceName(AllianceName),
    AllianceRankId(AllianceRankId),
    AllianceSearching(AllianceSearching),
    TimerProtectionTime(TimerProtectionTime),
    TimerRelocateTime(TimerRelocateTime),
    FactionFactionId(FactionFactionId),
    FactionTitleId(FactionTitleId),
    FactionSelfProtectionTime(FactionSelfProtectionTime),
    FactionGroupProtectionStatus(FactionGroupProtectionStatus),
    FactionGroupProtectionTime(FactionGroupProtectionTime),
    FactionMainCampId(FactionMainCampId),
    FactionSpecialCampId(FactionSpecialCampId),
}

impl Comparator {
    /// Returns the `Field` that the `Comparator` uses.
    pub(super) fn field(&self) -> Field {
        match self {
            Comparator::Header(_) => Field::Header,
            Comparator::Timestamp(_) => Field::Timestamp,
            Comparator::BasicName(_) => Field::Basic(BasicField::Name),
            Comparator::BasicLevel(_) => Field::Basic(BasicField::Level),
            Comparator::BasicLegendaryLevel(_) => Field::Basic(BasicField::LegendaryLevel),
            Comparator::BasicMight(_) => Field::Basic(BasicField::Might),
            Comparator::BasicHonor(_) => Field::Basic(BasicField::Honor),
            Comparator::BasicAchievement(_) => Field::Basic(BasicField::Achievement),
            Comparator::BasicGlory(_) => Field::Basic(BasicField::Glory),
            Comparator::BasicRuins(_) => Field::Basic(BasicField::Ruins),
            Comparator::AllianceId(_) => Field::Alliance(AllianceField::Id),
            Comparator::AllianceName(_) => Field::Alliance(AllianceField::Name),
            Comparator::AllianceRankId(_) => Field::Alliance(AllianceField::RankId),
            Comparator::AllianceSearching(_) => Field::Alliance(AllianceField::Searching),
            Comparator::TimerProtectionTime(_) => Field::Timer(TimerField::ProtectionTime),
            Comparator::TimerRelocateTime(_) => Field::Timer(TimerField::RelocateTime),
            Comparator::FactionFactionId(_) => Field::Faction(FactionField::FactionId),
            Comparator::FactionTitleId(_) => Field::Faction(FactionField::TitleId),
            Comparator::FactionSelfProtectionTime(_) => {
                Field::Faction(FactionField::SelfProtectionTime)
            }
            Comparator::FactionGroupProtectionStatus(_) => {
                Field::Faction(FactionField::GroupProtectionStatus)
            }
            Comparator::FactionGroupProtectionTime(_) => {
                Field::Faction(FactionField::GroupProtectionTime)
            }
            Comparator::FactionMainCampId(_) => Field::Faction(FactionField::MainCampId),
            Comparator::FactionSpecialCampId(_) => Field::Faction(FactionField::SpecialCampId),
        }
    }

    fn compare(&self, value: &Data, other: &Data) -> Option<Ordering> {
        Some(match (value, other) {
            (Data::Header(v), Data::Header(o)) => match self {
                Comparator::Header(c) => c.compare(v, o),
                _ => return None,
            },
            (Data::Timestamp(v), Data::Timestamp(o)) => match self {
                Comparator::Timestamp(c) => c.compare(v, o),
                _ => return None,
            },
            (Data::I64(v), Data::I64(o)) => match self {
                Comparator::BasicLevel(c) => c.compare(v, o),
                Comparator::BasicLegendaryLevel(c) => c.compare(v, o),
                Comparator::BasicMight(c) => c.compare(v, o),
                Comparator::BasicHonor(c) => c.compare(v, o),
                Comparator::BasicAchievement(c) => c.compare(v, o),
                Comparator::BasicGlory(c) => c.compare(v, o),
                Comparator::BasicRuins(c) => c.compare(v, o),
                Comparator::AllianceId(c) => c.compare(v, o),
                Comparator::AllianceRankId(c) => c.compare(v, o),
                Comparator::AllianceSearching(c) => c.compare(v, o),
                Comparator::TimerProtectionTime(c) => c.compare(v, o),
                Comparator::TimerRelocateTime(c) => c.compare(v, o),
                Comparator::FactionFactionId(c) => c.compare(v, o),
                Comparator::FactionTitleId(c) => c.compare(v, o),
                Comparator::FactionSelfProtectionTime(c) => c.compare(v, o),
                Comparator::FactionGroupProtectionStatus(c) => c.compare(v, o),
                Comparator::FactionGroupProtectionTime(c) => c.compare(v, o),
                Comparator::FactionMainCampId(c) => c.compare(v, o),
                Comparator::FactionSpecialCampId(c) => c.compare(v, o),
                _ => return None,
            },
            (Data::String(v), Data::String(o)) => match self {
                Comparator::BasicName(c) => c.compare(v, o),
                Comparator::AllianceName(c) => c.compare(v, o),
                _ => return None,
            },
            _ => return None,
        })
    }
}

pub(super) struct ComparisonEngine<'a> {
    comparators: &'a [Comparator],
    schema: Schema,
}

impl<'a> ComparisonEngine<'a> {
    pub(super) fn new(comparators: &'a [Comparator]) -> Self {
        let schema = Schema::from_comparators(comparators);
        ComparisonEngine {
            comparators,
            schema,
        }
    }

    pub(super) fn compare(&self, value: &[Data], other: &[Data]) -> Ordering {
        value
            .iter()
            .zip(other)
            .zip(self.comparators)
            // Using unwrap as compare shouldn't return None if both values have the correct variant
            .map(|((a, b), c)| c.compare(a, b).unwrap())
            // Equivalent to comparing lexicographically
            .find(|&ordering| ordering != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }

    pub(super) fn schema(&self) -> &Schema {
        &self.schema
    }
}
