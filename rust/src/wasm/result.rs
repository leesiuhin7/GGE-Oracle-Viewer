use wasm_bindgen::prelude::*;

use crate::{query, sorting, wasm::file_wrapper::FileReader};

#[wasm_bindgen]
pub struct SnapshotInfo(query::SnapshotInfo);

impl From<SnapshotInfo> for query::SnapshotInfo {
    fn from(value: SnapshotInfo) -> Self {
        value.0
    }
}

#[wasm_bindgen]
pub struct MatchResult(query::MatchResult);

#[wasm_bindgen]
impl MatchResult {
    pub fn get(&self, skip: usize, take: usize) -> Vec<SnapshotInfo> {
        self.0
            .get(skip, take)
            .into_iter()
            .map(SnapshotInfo)
            .collect()
    }

    pub fn clone(&self) -> MatchResult {
        MatchResult(self.0.clone())
    }
}

impl From<query::MatchResult> for MatchResult {
    fn from(value: query::MatchResult) -> Self {
        MatchResult(value)
    }
}

impl From<MatchResult> for query::MatchResult {
    fn from(value: MatchResult) -> Self {
        value.0
    }
}

#[wasm_bindgen]
pub struct SortingResult(sorting::SortingResult<FileReader>);

#[wasm_bindgen]
impl SortingResult {
    pub fn get(&mut self, skip: usize, take: usize) -> Option<Vec<SnapshotInfo>> {
        let start = u64::try_from(skip).ok()?;
        let end = u64::try_from(skip + take).ok()?;
        Some(
            self.0
                .get(start..end)
                .ok()?
                .into_iter()
                .map(SnapshotInfo)
                .collect(),
        )
    }
}

impl From<sorting::SortingResult<FileReader>> for SortingResult {
    fn from(value: sorting::SortingResult<FileReader>) -> Self {
        SortingResult(value)
    }
}
