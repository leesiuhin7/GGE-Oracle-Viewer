use crate::{
    data::{Block, block::Error},
    filter::{
        Filter, IntervalSet,
        interval::{intersect_interval_sets, interval_sets_union},
    },
};

pub enum Expr {
    And(Vec<Expr>),
    Or(Vec<Expr>),
    Filter(Filter),
}

impl Expr {
    pub fn eval(&self, block: &mut Block) -> Result<IntervalSet, Error> {
        match self {
            Expr::And(exprs) => Ok(intersect_interval_sets(
                exprs
                    .iter()
                    .map(|expr| expr.eval(block))
                    .collect::<Result<_, _>>()?,
            )),
            Expr::Or(exprs) => Ok(interval_sets_union(
                exprs
                    .iter()
                    .map(|expr| expr.eval(block))
                    .collect::<Result<_, _>>()?,
            )),
            Expr::Filter(filter) => filter.apply(block),
        }
    }
}
