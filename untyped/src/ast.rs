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
