use std::collections::HashMap;

use crate::data::filter::Filter;

pub struct FilterStorage {
    map: HashMap<u32, Filter>,
    counter: u32,
}

impl FilterStorage {
    pub(super) fn new() -> Self {
        FilterStorage {
            map: HashMap::new(),
            counter: 0,
        }
    }

    pub fn push(&mut self, filter: Filter) -> u32 {
        let counter = self.counter;
        self.counter += 1;
        self.map.insert(counter, filter);
        counter
    }

    pub fn remove(&mut self, id: u32) -> Option<Filter> {
        self.map.remove(&id)
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &Filter> {
        self.map.values()
    }
}
