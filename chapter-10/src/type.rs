use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Type {
    Bool,
    Fun(Box<Type>, Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
        | Type::Bool => write!(fmt, "bool"),
        | Type::Fun(from, to) => write!(fmt, "{} -> {}", from, to),
        }
    }
}
