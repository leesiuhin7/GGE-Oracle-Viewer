mod engine;
mod interval;

use crate::data::block::{Block, Data, Error, Field};
use crate::data::filter::interval::IntervalSet;

struct Filter {
    field: Field,
    predicate: Box<dyn Fn(Data) -> IntervalSet>,
}

impl Filter {
    fn new(field: Field, predicate: Box<dyn Fn(Data) -> IntervalSet>) -> Self {
        Filter { field, predicate }
    }

    fn apply(&self, block: &mut Block) -> Result<IntervalSet, Error> {
        let data = block.read_field(&self.field)?;
        Ok((self.predicate)(data))
    }
}
