use std::collections::HashMap;
use std::fmt;

use anyhow::anyhow;

use crate::term::Term;

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

#[derive(Clone, Debug, Default)]
pub struct Context(HashMap<i64, Type>);

impl<'a> Term<'a> {
    pub fn check(&self, context: &mut Context, depth: i64) -> anyhow::Result<Type> {
        match self {
        | Term::Bool(_) => Ok(Type::Bool),
        | Term::If { r#if, then, r#else } => {
            if r#if.check(context, depth)? != Type::Bool {
                return Err(anyhow!("Guard of conditional not a boolean"));
            }
            let then_type = then.check(context, depth)?;
            let else_type = r#else.check(context, depth)?;
            if then_type != else_type {
                return Err(anyhow!("Arms of conditional have different types"));
            }
            Ok(then_type)
        }
        | Term::Var { index } => context.0
            .get(index)
            .cloned()
            .ok_or_else(|| anyhow!("Unbound variable: {}", index)),
        | Term::Abs { hint: _, r#type, term } => {
            context.0.insert(depth, r#type.clone());
            let term_type = term.check(context, depth + 1)?;
            context.0.remove(&depth);
            Ok(Type::Fun(Box::new(r#type.clone()), Box::new(term_type)))
        }
        | Term::App { fun, arg } => {
            let fun_type = fun.check(context, depth)?;
            let arg_type = arg.check(context, depth)?;
            match fun_type {
            | Type::Fun(expected_arg_type, return_type) if *expected_arg_type == arg_type => Ok(*return_type),
            | Type::Fun(_, _) => Err(anyhow!("Parameter type mismatch")),
            | Type::Bool => Err(anyhow!("Function type expected")),
            }
        }
        }
    }
}
