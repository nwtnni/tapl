use std::iter;

use crate::ast;

impl ast::Exp {
    pub fn step_all(self) -> impl Iterator<Item = ast::Exp> {
        iter::successors(Some(self), |prev| prev.step())
    }

    pub fn step(&self) -> Option<ast::Exp> {
        use ast::Exp::*;
        let next = match self {
        | Cond(box True, box t, _) => t.clone(),
        | Cond(box False, _, box f) => f.clone(),
        | Cond(b, t, f) => Cond(Box::new(b.step()?), t.clone(), f.clone()),
        | Succ(n) => Succ(Box::new(n.step()?)),
        | Pred(box Zero) => Zero,
        | Pred(box Succ(box n)) => n.clone(),
        | Pred(n) => Pred(Box::new(n.step()?)),
        | IsZero(box Zero) => True,
        | IsZero(box Succ(box n)) if n.is_numeric() => False,
        | IsZero(n) => IsZero(Box::new(n.step()?)),
        | _ => return None,
        };
        Some(next)
    }

    #[allow(unused)]
    pub fn eval(&self) -> Option<ast::Exp> {
        use ast::Exp::*;
        let value = match self {
        | Cond(b, t, f) => {
            match b.eval()? {
            | True => t.eval()?,
            | False => f.eval()?,
            | _ => return None,
            }
        }
        | Succ(n) => Succ(Box::new(n.eval()?)),
        | Pred(n) => {
            match n.eval()? {
            | Zero => Zero,
            | Succ(n) => *n,
            | _ => return None,
            }
        }
        | IsZero(n) => {
            match n.eval()? {
            | Zero => True,
            | Succ(_) => False,
            | _ => return None,
            }
        }
        | value => value.clone(),
        };
        Some(value)
    }
}
