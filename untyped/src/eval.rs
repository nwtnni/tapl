use crate::ast;

impl ast::Exp {
    pub fn step(self) -> Result<ast::Exp, ast::Exp> {
        use ast::Exp::*;
        let next = match self {
        | Cond(box True, t, _) => *t,
        | Cond(box False, _, f) => *f,
        | Cond(b, t, f) => {
            let b = b.step()?;
            Cond(Box::new(b), t, f)
        }
        | Succ(n) => {
            let n = n.step()?;
            Succ(Box::new(n))
        }
        | Pred(box Zero) => Zero,
        | Pred(box Succ(n)) => *n,
        | IsZero(box Zero) => True,
        | IsZero(box Succ(ref n)) if n.is_numeric() => False,
        | IsZero(n) => {
            let n = n.step()?;
            IsZero(Box::new(n))
        }
        | stuck => return Err(stuck),
        };
        Ok(next)
    }
}
