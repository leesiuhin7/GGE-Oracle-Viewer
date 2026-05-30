use crate::data::{
    block::{Block, Error},
    filter::{
        Filter,
        interval::{IntervalSet, intersect_interval_sets},
    },
};

pub struct Engine {
    filters: Vec<Filter>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            filters: Vec::new(),
        }
    }

    pub fn filters_mut(&mut self) -> &mut Vec<Filter> {
        &mut self.filters
    }

    pub fn apply_filters(&self, block: &mut Block) -> Result<IntervalSet, Error> {
        let interval_sets: Vec<IntervalSet> = self
            .filters
            .iter()
            .map(|filter| filter.apply(block))
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(intersect_interval_sets(interval_sets))
    }
}
