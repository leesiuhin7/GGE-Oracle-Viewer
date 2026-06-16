use crate::filter::{Interval, IntervalSet};

pub struct SnapshotInfo {
    pub(super) block_id: usize,
    pub(super) snapshot_id: u32,
}

struct BlockIntervalSet {
    block_id: usize,
    interval_set: IntervalSet,
}

pub struct MatchResult {
    interval_sets: Vec<BlockIntervalSet>,
}

impl MatchResult {
    pub(super) fn new() -> Self {
        MatchResult {
            interval_sets: Vec::new(),
        }
    }

    pub(super) fn add(&mut self, block_id: usize, interval_set: IntervalSet) {
        self.interval_sets.push(BlockIntervalSet {
            block_id,
            interval_set,
        });
    }

    pub fn get(&self, skip: usize, take: usize) -> Vec<SnapshotInfo> {
        self.interval_sets
            .iter()
            .flat_map(
                |BlockIntervalSet {
                     block_id,
                     interval_set,
                 }| {
                    interval_set
                        .iter()
                        .filter_map(|&Interval { start, end }| {
                            let block_id = *block_id;
                            if start == 0 && end == u32::MAX {
                                None
                            } else {
                                Some((start..end).map(move |snapshot_id| SnapshotInfo {
                                    block_id,
                                    snapshot_id,
                                }))
                            }
                        })
                        .flatten()
                },
            )
            .skip(skip)
            .take(take)
            .collect()
    }
}
