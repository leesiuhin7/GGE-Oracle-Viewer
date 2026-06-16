use wasm_bindgen::prelude::*;

use crate::{
    data::BlockReader,
    query,
    wasm::{
        expr_wrapper::Expr,
        fields::WrapperField,
        file_wrapper::{FileReader, FileWrapper, SyncFile},
        layout::LayoutWrapper,
        match_result::{MatchResult, SnapshotInfo},
        snapshot_data::Snapshot,
    },
};

#[wasm_bindgen]
pub struct Engine(query::Engine<FileReader>);

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(sync_file: SyncFile, layout_wrapper: LayoutWrapper) -> Self {
        let reader = FileReader::new(FileWrapper::new(sync_file));
        Engine(query::Engine::new(BlockReader::new(
            reader,
            layout_wrapper.into_layout(),
        )))
    }

    pub fn match_all(&mut self, expr: Expr) -> MatchResult {
        self.0.match_all(&expr.into()).into()
    }

    #[allow(clippy::needless_pass_by_value)]
    pub fn build_snapshot(
        &mut self,
        snapshot_info: SnapshotInfo,
        fields: Vec<WrapperField>,
    ) -> Option<Snapshot> {
        self.0
            .build_snapshot(
                &snapshot_info.into(),
                fields.into_iter().map(std::convert::Into::into).collect(),
            )
            .map(std::convert::Into::into)
    }
}
