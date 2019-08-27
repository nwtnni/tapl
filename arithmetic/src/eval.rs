use crate::ast;

impl ast::Exp {
    pub fn step_all(mut self) -> Result<ast::Exp, ast::Exp> {
        loop {
            match self.step() {
            | Ok(next) => self = next,
            | Err(stuck) => break if stuck.is_value() { Ok(stuck) } else { Err(stuck) },
            }
        }
    }

    pub fn step(self) -> Result<ast::Exp, ast::Exp> {
        use ast::Exp::*;
        let next = match self {
        | Cond(box True, t, _) => *t,
        | Cond(box False, _, f) => *f,
        | Cond(b, t, f) => Cond(Box::new(b.step()?), t, f),
        | Succ(n) => Succ(Box::new(n.step()?)),
        | Pred(box Zero) => Zero,
        | Pred(box Succ(n)) => *n,
        | IsZero(box Zero) => True,
        | IsZero(box Succ(n)) if n.is_numeric() => False,
        | IsZero(n) => IsZero(Box::new(n.step()?)),
        | stuck => return Err(stuck),
        };
        Ok(next)
    }

    pub fn eval(self) -> Result<ast::Exp, ast::Exp> {
        use ast::Exp::*;
        let value = match self {
        | Cond(b, t, f) => {
            match b.eval()? {
            | True => t.eval()?,
            | False => f.eval()?,
            | stuck => return Err(stuck),
            }
        }
        | Succ(n) => Succ(Box::new(n.eval()?)),
        | Pred(n) => {
            match n.eval()? {
            | Zero => Zero,
            | Succ(n) => *n,
            | stuck => return Err(stuck)
            }
        }
        | IsZero(n) => {
            match n.eval()? {
            | Zero => True,
            | Succ(_) => False,
            | stuck => return Err(stuck)
            }
        }
        | value => value,
        };
        Ok(value)
    }
}
