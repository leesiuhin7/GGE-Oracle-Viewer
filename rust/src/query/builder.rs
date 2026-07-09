use crate::{
    data::{
        Block,
        block::{Data, Field},
        structures::{delta_rle, rle},
    },
    query::snapshot::{Snapshot, SnapshotData},
};

fn resolve_rle<T>(runs: Vec<rle::Run<T>>, snapshot_id: u32) -> Option<T> {
    let mut counter = 0u64;
    for rle::Run { value, count } in runs {
        counter += count;
        if u64::from(snapshot_id) < counter {
            return Some(value);
        }
    }
    None
}

enum DeltaValue {
    Failed,
    None,
    Value(i64),
}

fn resolve_delta_rle(deltas: Vec<delta_rle::Run>, snapshot_id: u32) -> DeltaValue {
    let mut value = 0i64;
    let mut counter = 0u64;
    for delta_rle::Run { delta, count } in deltas {
        if u64::from(snapshot_id) < counter + count {
            let Some(d) = delta else {
                return DeltaValue::None;
            };
            // Adding 1 to make n from 0..count to 1..=count
            if let Ok(n) = i64::try_from(u64::from(snapshot_id) - counter + 1) {
                value += d * n;
                return DeltaValue::Value(value);
            }
            return DeltaValue::Failed;
        }
        if let Some(d) = delta {
            let Ok(c) = i64::try_from(count) else {
                return DeltaValue::Failed;
            };
            value += d * c;
        }
        counter += count;
    }
    DeltaValue::Failed
}

fn resolve_timestamp(timestamps: &[i64], snapshot_id: u32) -> Option<i64> {
    timestamps.get(snapshot_id as usize).copied()
}

fn resolve_field(data: Data, snapshot_id: u32) -> Option<SnapshotData> {
    match data {
        Data::Header(header) => Some(SnapshotData::Header(header)),
        Data::Deltas(_) => unreachable!(),
        Data::Timestamps(timestamps) => Some(SnapshotData::Timestamp(resolve_timestamp(
            &timestamps,
            snapshot_id,
        )?)),
        Data::RleDelta(deltas) => match resolve_delta_rle(deltas, snapshot_id) {
            DeltaValue::Failed => None,
            DeltaValue::None => Some(SnapshotData::I64(None)),
            DeltaValue::Value(v) => Some(SnapshotData::I64(Some(v))),
        },
        Data::RleI64(rle_i64) => Some(SnapshotData::I64(resolve_rle(rle_i64, snapshot_id)?)),
        Data::RleString(rle_string) => {
            Some(SnapshotData::String(resolve_rle(rle_string, snapshot_id)?))
        }
        Data::RleLocations(rle_locations) => Some(SnapshotData::Locations(resolve_rle(
            rle_locations,
            snapshot_id,
        )?)),
        Data::RleCoatOfArms(rle_coat_of_arms) => Some(SnapshotData::CoatOfArms(resolve_rle(
            rle_coat_of_arms,
            snapshot_id,
        )?)),
    }
}

pub(super) struct SnapshotBuilder<'a> {
    block: Block<'a>,
}

impl<'a> SnapshotBuilder<'a> {
    pub(super) fn new(block: Block<'a>) -> Self {
        SnapshotBuilder { block }
    }

    pub(super) fn build_snapshot(
        &mut self,
        snapshot_id: u32,
        fields: &[Field],
    ) -> Option<Snapshot> {
        let mut snapshot = Snapshot::new();
        for field in fields {
            let Ok(data) = self.block.read_field(field) else {
                return None;
            };
            let snapshot_data = resolve_field(data, snapshot_id)?;
            snapshot.set(field, snapshot_data).ok()?;
        }
        Some(snapshot)
    }
}
