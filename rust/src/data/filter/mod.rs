use crate::data::block::{Block, Data, Error, Field};
pub use crate::data::filter::{
    builder::build_filter,
    engine::Engine,
    interval::{Interval, IntervalSet},
};

mod builder;
mod engine;
pub mod field_filters;
mod interval;
mod storage;

pub struct Filter {
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
