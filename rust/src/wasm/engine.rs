use wasm_bindgen::prelude::*;

use crate::{
    data::{
        BlockReader, Engine as FilterEngine,
        filter::{Interval, build_filter},
    },
    wasm::{
        fields::WrapperField,
        file_wrapper::{FileReader, FileWrapper, SyncFile},
        filter_wrapper::Filter,
        layout::LayoutWrapper,
        snapshot::SnapshotBuilder,
        snapshot_data::Snapshot,
    },
};

#[wasm_bindgen]
pub struct SnapshotInfo {
    block_id: usize,
    snapshot_id: u32,
}

#[wasm_bindgen]
pub struct Engine {
    block_reader: BlockReader<FileReader>,
    filter_engine: FilterEngine,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(sync_file: SyncFile, layout_wrapper: LayoutWrapper) -> Self {
        let reader = FileReader::new(FileWrapper::new(sync_file));
        Engine {
            block_reader: BlockReader::new(reader, layout_wrapper.into_layout()),
            filter_engine: FilterEngine::new(),
        }
    }

    pub fn push_filter(&mut self, filter: Filter) -> u32 {
        self.filter_engine
            .storage_mut()
            .push(build_filter(filter.into()))
    }

    pub fn remove_filter(&mut self, id: u32) -> bool {
        self.filter_engine.storage_mut().remove(id).is_some()
    }

    pub fn match_all(&mut self, skip: usize, take: usize) -> Vec<SnapshotInfo> {
        self.block_reader
            .blocks()
            // Map blocks to intervals
            .map(|block| match block {
                Ok(mut b) => self.filter_engine.apply_filters(&mut b).ok(),
                Err(_) => None,
            })
            // Assign id to each block
            .enumerate()
            .filter_map(|(block_id, interval_set)| interval_set.map(|set| (block_id, set)))
            // Convert intervals into snapshots
            .flat_map(|(block_id, interval_set)| {
                let mut snapshots = Vec::new();
                for Interval { start, end } in interval_set {
                    if !(start == 0 && end == u32::MAX) {
                        for snapshot_id in start..end {
                            snapshots.push(SnapshotInfo {
                                block_id,
                                snapshot_id,
                            });
                        }
                    }
                }
                snapshots
            })
            .skip(skip)
            .take(take)
            .collect::<Vec<_>>()
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
