#[derive(Clone, Debug)]
pub enum Exp {
    True,
    False,
    Zero,
    Cond(Box<Exp>, Box<Exp>, Box<Exp>),
    Succ(Box<Exp>),
    Pred(Box<Exp>),
    IsZero(Box<Exp>),
}

impl Exp {
    pub fn is_numeric(&self) -> bool {
        match self {
        | Exp::Zero => true,
        | Exp::Succ(n) => n.is_numeric(),
        | _ => false,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
        | Exp::True
        | Exp::False => true,
        | n => n.is_numeric(),
        }
    }
}
