use std::fmt;

use crate::ast;

impl fmt::Display for ast::Exp {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        use ast::Exp::*;
        match self {
        | True => write!(fmt, "true"), 
        | False => write!(fmt, "false"),
        | Zero => write!(fmt, "0"),
        | Cond(b, t, f) => write!(fmt, "if {} then {} else {}", b, t, f),
        | Succ(n) => write!(fmt, "succ {}", n),
        | Pred(n) => write!(fmt, "pred {}", n),
        | IsZero(n) => write!(fmt, "iszero {}", n),
        }
    }
}
