use wasm_bindgen::prelude::*;

use crate::{
    query,
    sorting::{self, comparator},
    wasm::{
        comparator::Comparator,
        engine_factory::{Producer, QueryEngine, SortingEngine, query_engine, sorting_engine},
        expr_wrapper::Expr,
        fields::WrapperField,
        file_wrapper::{FileReader, FileWrapper, FileWriter, SyncFile},
        layout::LayoutWrapper,
        result::{MatchResult, SnapshotInfo, SortingResult},
        snapshot_data::Snapshot,
    },
};

#[wasm_bindgen]
pub struct Files {
    data: SyncFile,
    temp1: SyncFile,
    temp2: SyncFile,
}

#[wasm_bindgen]
impl Files {
    #[wasm_bindgen(constructor)]
    pub fn new(data: SyncFile, temp1: SyncFile, temp2: SyncFile) -> Self {
        Files { data, temp1, temp2 }
    }
}

#[wasm_bindgen]
pub struct Engine {
    query_engine: QueryEngine,
    sorting_engine: SortingEngine,
}

#[wasm_bindgen]
impl Engine {
    #[wasm_bindgen(constructor)]
    pub fn new(files: Files, layout_wrapper: LayoutWrapper) -> Self {
        let Files { data, temp1, temp2 } = files;
        Engine {
            query_engine: query_engine(data, layout_wrapper),
            sorting_engine: sorting_engine(temp1, temp2),
        }
    }

    pub fn match_all(&mut self, expr: Expr) -> MatchResult {
        self.query_engine.match_all(&expr.into()).into()
    }

    pub fn sort(
        &mut self,
        result: MatchResult,
        comparators: Vec<Comparator>,
        file: SyncFile,
    ) -> Option<SortingResult> {
        let match_result = query::MatchResult::from(result);
        let iter = query::MatchResult::iter(&match_result);

        let producer = Producer::new(&mut self.query_engine);

        let mut file_wrapper = FileWrapper::new(file);
        file_wrapper.truncate(0); // Clear file to start fresh
        let mut output = FileWriter::new(file_wrapper);

        self.sorting_engine
            .sort(
                iter,
                producer,
                &comparators
                    .into_iter()
                    .map(std::convert::Into::into)
                    .collect::<Vec<comparator::Comparator>>(),
                &mut output,
            )
            .ok()?;

        let reader = FileReader::new(output.into_wrapper().ok()?);
        Some(sorting::SortingResult::new(reader).into())
    }

    pub fn build_snapshot(
        &mut self,
        snapshot_info: SnapshotInfo,
        fields: Vec<WrapperField>,
    ) -> Option<Snapshot> {
        self.query_engine
            .build_snapshot(
                &snapshot_info.into(),
                &fields
                    .into_iter()
                    .map(std::convert::Into::into)
                    .collect::<Vec<_>>(),
            )
            .map(std::convert::Into::into)
    }
}
