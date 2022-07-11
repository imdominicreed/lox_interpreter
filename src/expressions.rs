
use std::any::Any;
use crate::token::{Token, LiteralObject};
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping {
        expresison: Box<Expr>,
    },
    Literal {
        value: Box<dyn Any>,
    },
    Unary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    }
}

