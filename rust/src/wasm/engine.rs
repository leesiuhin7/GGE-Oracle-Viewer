use wasm_bindgen::prelude::*;

use crate::{
    data::BlockReader,
    wasm::{
        expr_wrapper::Expr,
        fields::WrapperField,
        file_wrapper::{FileReader, FileWrapper, SyncFile},
        layout::LayoutWrapper,
        match_result::{MatchResult, SnapshotInfo},
        snapshot::SnapshotBuilder,
        snapshot_data::Snapshot,
    },
};

#[wasm_bindgen]
pub struct Engine {
    block_reader: BlockReader<FileReader>,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(sync_file: SyncFile, layout_wrapper: LayoutWrapper) -> Self {
        let reader = FileReader::new(FileWrapper::new(sync_file));
        Engine {
            block_reader: BlockReader::new(reader, layout_wrapper.into_layout()),
        }
    }

    pub fn match_all(&mut self, expr: Expr) -> MatchResult {
        let filter_expr = expr.into_expr();

        let mut result = MatchResult::new();

        for (block_id, interval_set) in self
            .block_reader
            .blocks()
            // Map blocks to intervals
            .map(|block| match block {
                Ok(mut b) => filter_expr.eval(&mut b).ok(),
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

    #[allow(clippy::needless_pass_by_value)]
    pub fn build_snapshot(
        &mut self,
        snapshot_info: SnapshotInfo,
        fields: Vec<WrapperField>,
    ) -> Option<Snapshot> {
        let SnapshotInfo {
            block_id,
            snapshot_id,
        } = snapshot_info;
        let block = self.block_reader.get_block(block_id).ok()?;
        let mut snapshot_builder = SnapshotBuilder::new(block);
        snapshot_builder.build_snapshot(
            snapshot_id,
            fields.into_iter().map(std::convert::Into::into).collect(),
        )
    }
}
