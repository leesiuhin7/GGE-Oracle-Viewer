use crate::{
    query::SnapshotInfo,
    sorting::{comparator::ComparisonEngine, data::Data},
};

pub(super) struct Item<'a> {
    keys: Vec<Data>,
    pub(super) info: SnapshotInfo,
    pub(super) index: usize,
    engine: &'a ComparisonEngine<'a>,
}

impl<'a> Item<'a> {
    pub(super) fn new(
        keys: Vec<Data>,
        info: SnapshotInfo,
        index: usize,
        engine: &'a ComparisonEngine,
    ) -> Self {
        Item {
            keys,
            info,
            index,
            engine,
        }
    }
}

impl PartialOrd for Item<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for Item<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.engine.compare(&self.keys, &other.keys).is_eq()
    }
}

impl Eq for Item<'_> {}

impl Ord for Item<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.engine.compare(&self.keys, &other.keys)
    }
}
