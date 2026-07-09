use crate::{
    data::block::Field,
    query::{Snapshot, SnapshotInfo},
};

pub(crate) trait Producer {
    fn produce(&mut self, snapshot_info: &SnapshotInfo, fields: &[Field]) -> Option<Snapshot>;
}
