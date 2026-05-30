use crate::data::{
    block::{Block, Error},
    filter::{
        interval::{IntervalSet, intersect_interval_sets},
        storage::FilterStorage,
    },
};

pub struct Engine {
    storage: FilterStorage,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            storage: FilterStorage::new(),
        }
    }

    pub fn storage_mut(&mut self) -> &mut FilterStorage {
        &mut self.storage
    }

    pub fn apply_filters(&self, block: &mut Block) -> Result<IntervalSet, Error> {
        let interval_sets: Vec<IntervalSet> = self
            .storage
            .iter()
            .map(|filter| filter.apply(block))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(intersect_interval_sets(interval_sets))
    }
}
