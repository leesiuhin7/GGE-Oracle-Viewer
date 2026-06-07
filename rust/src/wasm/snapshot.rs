use crate::{
    data::{
        Block,
        block::{Data, Field},
        structures::{delta_rle, rle},
    },
    wasm::snapshot_data::{Snapshot, SnapshotData},
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
        counter += count;
        if u64::from(snapshot_id) < counter {
            let Some(d) = delta else {
                return DeltaValue::None;
            };
            if let Ok(n) = i64::try_from(counter - u64::from(snapshot_id)) {
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
    }
    DeltaValue::Failed
}

fn resolve_timestamp(deltas: Vec<i64>, snapshot_id: u32) -> Option<i64> {
    if deltas.len() > snapshot_id as usize {
        None
    } else {
        Some(deltas.into_iter().take(snapshot_id as usize).sum())
    }
}

fn resolve_field(data: Data, snapshot_id: u32) -> Option<SnapshotData> {
    match data {
        Data::Header(header) => Some(SnapshotData::Header(header)),
        Data::Deltas(_) => unreachable!(),
        Data::Timestamps(deltas) => Some(SnapshotData::Timestamp(resolve_timestamp(
            deltas,
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

pub struct SnapshotBuilder<'a> {
    block: Block<'a>,
}

impl<'a> SnapshotBuilder<'a> {
    pub fn new(block: Block<'a>) -> Self {
        SnapshotBuilder { block }
    }

    pub fn build_snapshot(&mut self, snapshot_id: u32, fields: Vec<Field>) -> Option<Snapshot> {
        let mut snapshot = Snapshot::new();
        for field in fields {
            let Ok(data) = self.block.read_field(&field) else {
                return None;
            };
            let snapshot_data = resolve_field(data, snapshot_id)?;
            snapshot.set(field, snapshot_data).ok()?;
        }
        Some(snapshot)
    }
}
