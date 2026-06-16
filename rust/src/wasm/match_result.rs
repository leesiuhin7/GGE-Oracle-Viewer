use wasm_bindgen::prelude::*;

use crate::query;

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
}

impl From<query::MatchResult> for MatchResult {
    fn from(value: query::MatchResult) -> Self {
        MatchResult(value)
    }
}
