use wasm_bindgen::prelude::*;

use crate::{filter, wasm::filter_wrapper::Filter};

pub enum ExprWrapper {
    And(Vec<ExprWrapper>),
    Or(Vec<ExprWrapper>),
    Filter(Filter),
}

impl From<ExprWrapper> for filter::Expr {
    fn from(value: ExprWrapper) -> Self {
        match value {
            ExprWrapper::And(exprs) => {
                filter::Expr::And(exprs.into_iter().map(std::convert::Into::into).collect())
            }
            ExprWrapper::Or(exprs) => {
                filter::Expr::Or(exprs.into_iter().map(std::convert::Into::into).collect())
            }
            ExprWrapper::Filter(filter) => {
                filter::Expr::Filter(filter::build_filter(filter.into()))
            }
        }
    }
}

#[wasm_bindgen]
pub struct Expr(ExprWrapper);

#[wasm_bindgen]
impl Expr {
    pub fn and(exprs: Vec<Expr>) -> Expr {
        Expr(ExprWrapper::And(
            exprs.into_iter().map(|expr_struct| expr_struct.0).collect(),
        ))
    }

    pub fn or(exprs: Vec<Expr>) -> Expr {
        Expr(ExprWrapper::Or(
            exprs.into_iter().map(|expr_struct| expr_struct.0).collect(),
        ))
    }

    pub fn filter(filter: Filter) -> Expr {
        Expr(ExprWrapper::Filter(filter))
    }

    pub(super) fn into_expr(self) -> filter::Expr {
        self.0.into()
    }
}
