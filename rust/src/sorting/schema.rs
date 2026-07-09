use std::collections::HashSet;

use crate::{
    data::block::{AllianceField, BasicField, FactionField, Field, TimerField},
    query::{Snapshot, SnapshotField},
    sorting::{comparator::Comparator, data::Data},
};

#[allow(clippy::too_many_lines)]
/// Extracts the value of the field from `snapshot`.
/// Returns None if the field is empty or if the data type is unsupported.
fn extract_field(snapshot: &mut Snapshot, field: &Field) -> Option<Data> {
    match field {
        Field::Header => match core::mem::take(&mut snapshot.header) {
            SnapshotField::Some(header) => Some(Data::Header(header)),
            SnapshotField::None => None,
        },
        Field::Timestamp => match core::mem::take(&mut snapshot.timestamp) {
            SnapshotField::Some(timestamp) => Some(Data::Timestamp(timestamp)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::Name) => match core::mem::take(&mut snapshot.basic_name) {
            SnapshotField::Some(name) => Some(Data::String(name)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::Level) => match core::mem::take(&mut snapshot.basic_level) {
            SnapshotField::Some(level) => Some(Data::I64(level)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::LegendaryLevel) => {
            match core::mem::take(&mut snapshot.basic_legendary_level) {
                SnapshotField::Some(legendary_level) => Some(Data::I64(legendary_level)),
                SnapshotField::None => None,
            }
        }
        Field::Basic(BasicField::Might) => match core::mem::take(&mut snapshot.basic_might) {
            SnapshotField::Some(might) => Some(Data::I64(might)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::Honor) => match core::mem::take(&mut snapshot.basic_honor) {
            SnapshotField::Some(honor) => Some(Data::I64(honor)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::Achievement) => {
            match core::mem::take(&mut snapshot.basic_achievement) {
                SnapshotField::Some(achievement) => Some(Data::I64(achievement)),
                SnapshotField::None => None,
            }
        }
        Field::Basic(BasicField::Glory) => match core::mem::take(&mut snapshot.basic_glory) {
            SnapshotField::Some(glory) => Some(Data::I64(glory)),
            SnapshotField::None => None,
        },
        Field::Basic(BasicField::Ruins) => match core::mem::take(&mut snapshot.basic_ruins) {
            SnapshotField::Some(ruins) => Some(Data::I64(ruins)),
            SnapshotField::None => None,
        },
        Field::Alliance(AllianceField::Id) => match core::mem::take(&mut snapshot.alliance_id) {
            SnapshotField::Some(id) => Some(Data::I64(id)),
            SnapshotField::None => None,
        },
        Field::Alliance(AllianceField::Name) => {
            match core::mem::take(&mut snapshot.alliance_name) {
                SnapshotField::Some(name) => Some(Data::String(name)),
                SnapshotField::None => None,
            }
        }
        Field::Alliance(AllianceField::RankId) => {
            match core::mem::take(&mut snapshot.alliance_rank_id) {
                SnapshotField::Some(rank_id) => Some(Data::I64(rank_id)),
                SnapshotField::None => None,
            }
        }
        Field::Alliance(AllianceField::Searching) => {
            match core::mem::take(&mut snapshot.alliance_searching) {
                SnapshotField::Some(searching) => Some(Data::I64(searching)),
                SnapshotField::None => None,
            }
        }
        Field::Timer(TimerField::ProtectionTime) => {
            match core::mem::take(&mut snapshot.timer_protection_time) {
                SnapshotField::Some(protection_time) => Some(Data::I64(protection_time)),
                SnapshotField::None => None,
            }
        }
        Field::Timer(TimerField::RelocateTime) => {
            match core::mem::take(&mut snapshot.timer_relocate_time) {
                SnapshotField::Some(relocate_time) => Some(Data::I64(relocate_time)),
                SnapshotField::None => None,
            }
        }

        Field::Location | Field::CoatOfArms => None,

        Field::Faction(FactionField::FactionId) => {
            match core::mem::take(&mut snapshot.faction_id) {
                SnapshotField::Some(faction_id) => Some(Data::I64(faction_id)),
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::TitleId) => {
            match core::mem::take(&mut snapshot.faction_title_id) {
                SnapshotField::Some(title_id) => Some(Data::I64(title_id)),
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::SelfProtectionTime) => {
            match core::mem::take(&mut snapshot.faction_self_protection_time) {
                SnapshotField::Some(self_protection_time) => Some(Data::I64(self_protection_time)),
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::GroupProtectionStatus) => {
            match core::mem::take(&mut snapshot.faction_group_protection_status) {
                SnapshotField::Some(group_protection_status) => {
                    Some(Data::I64(group_protection_status))
                }
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::GroupProtectionTime) => {
            match core::mem::take(&mut snapshot.faction_group_protection_time) {
                SnapshotField::Some(group_protection_time) => {
                    Some(Data::I64(group_protection_time))
                }
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::MainCampId) => {
            match core::mem::take(&mut snapshot.faction_main_camp_id) {
                SnapshotField::Some(main_camp_id) => Some(Data::I64(main_camp_id)),
                SnapshotField::None => None,
            }
        }
        Field::Faction(FactionField::SpecialCampId) => {
            match core::mem::take(&mut snapshot.faction_special_camp_id) {
                SnapshotField::Some(special_camp_id) => Some(Data::I64(special_camp_id)),
                SnapshotField::None => None,
            }
        }
    }
}

pub(super) struct Schema {
    fields: Vec<Field>,
}

impl Schema {
    pub(super) fn from_comparators(comparators: &[Comparator]) -> Self {
        let mut fields = Vec::new();
        let mut used_fields = HashSet::new();

        for comparator in comparators {
            let field = comparator.field();
            let field_id = field.as_usize();
            if !used_fields.contains(&field_id) {
                used_fields.insert(field_id);
                fields.push(field);
            }
        }
        Schema { fields }
    }

    pub(super) fn fields(&self) -> &Vec<Field> {
        &self.fields
    }

    pub(super) fn convert_snapshot(&self, mut snapshot: Snapshot) -> Vec<Data> {
        self.fields
            .iter()
            // Using unwrap as it should be impossible to receive None unless either:
            //  - The snapshot wasn't created correctly
            //  - The same field was extracted multiple times (duplicate fields)
            .map(|field| extract_field(&mut snapshot, field).unwrap())
            .collect()
    }
}
