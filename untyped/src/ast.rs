#[derive(Clone, Debug)]
pub enum Exp {
    Var(String),
    Abs(String, Box<Exp>),
    App(Box<Exp>, Box<Exp>),
}
