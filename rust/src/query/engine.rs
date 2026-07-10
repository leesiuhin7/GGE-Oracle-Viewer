use std::io::{Read, Seek};

use crate::{
    data::{BlockReader, block::Field},
    filter::Expr,
    query::{
        builder::SnapshotBuilder,
        result::{MatchResult, SnapshotInfo},
        snapshot::Snapshot,
    },
};

pub(crate) struct Engine<R: Read + Seek> {
    block_reader: BlockReader<R>,
}

impl<R: Read + Seek> Engine<R> {
    pub(crate) fn new(block_reader: BlockReader<R>) -> Self {
        Engine { block_reader }
    }

    pub(crate) fn match_all(&mut self, expr: &Expr) -> MatchResult {
        let mut result = MatchResult::new();

        for (block_id, interval_set) in self
            .block_reader
            .blocks()
            // Map blocks to intervals
            .map(|block| match block {
                Ok(mut b) => expr.eval(&mut b).ok(),
                Err(_) => None,
            })
            // Assign id to each block
            .enumerate()
            .filter_map(|(block_id, interval_set)| interval_set.map(|set| (block_id, set)))
        {
            result.add(block_id, interval_set);
        }
        result
    }

    pub(crate) fn build_snapshot(
        &mut self,
        snapshot_info: &SnapshotInfo,
        fields: &[Field],
    ) -> Option<Snapshot> {
        let &SnapshotInfo {
            block_id,
            snapshot_id,
        } = snapshot_info;
        let block = self.block_reader.get_block(block_id).ok()?;

        let mut builder = SnapshotBuilder::new(block);
        builder.build_snapshot(snapshot_id, fields)
    }
}
