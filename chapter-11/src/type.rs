use std::collections::HashMap;
use std::fmt;

use anyhow::anyhow;
use indexmap::IndexMap;

use crate::term::Term;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Bool,
    Fun(Box<Type>, Box<Type>),
    Tuple(Vec<Type>),
    Record(IndexMap<String, Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
        | Type::Bool => write!(fmt, "bool"),
        | Type::Fun(from, to) => write!(fmt, "{} -> {}", from, to),
        | Type::Tuple(types) => {
            let mut types = types.iter();
            write!(fmt, "(")?;
            if let Some(head) = types.next() {
                write!(fmt, "{}", head)?;
            }
            while let Some(tail) = types.next() {
                write!(fmt, ", {}", tail)?;
            }
            write!(fmt, ")")
        }
        | Type::Record(types) => {
            let mut types = types.iter();
            write!(fmt, "{{")?;
            if let Some((label, r#type)) = types.next() {
                write!(fmt, "{} = {}", label, r#type)?;
            }
            while let Some((label, r#type)) = types.next() {
                write!(fmt, ", {} = {}", label, r#type)?;
            }
            write!(fmt, "}}")
        }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct Context(HashMap<i64, Type>);

impl<'a> Term<'a> {
    pub fn check(&self, context: &mut Context, depth: i64) -> anyhow::Result<Type> {
        match self {
        // ---------------- T-True
        // Γ |- true : Bool
        | Term::Bool(true)
        // ----------------- T-False
        // Γ |- false : Bool
        | Term::Bool(false) => Ok(Type::Bool),

        // Γ |- t₁ : Bool    Γ |- t₂ : T    Γ |- t₃ : T
        // -------------------------------------------- T-If
        //          if t₁ then t₂ else t₃ : T
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

        // x : T ∈ Γ
        // ---------- T-Var
        // Γ |- x : T
        | Term::Var(index) => context.0
            .get(index)
            .cloned()
            .ok_or_else(|| anyhow!("Unbound variable: {}", index)),

        //    Γ, x : T₁ |- t₂ : T₂
        // --------------------------- T-Abs
        // Γ |- λx : T₁. t₂ : T₁ -> T₂
        | Term::Abs { hint: _, r#type, body } => {
            context.0.insert(depth, r#type.clone());
            let body_type = body.check(context, depth + 1)?;
            context.0.remove(&depth);
            Ok(Type::Fun(Box::new(r#type.clone()), Box::new(body_type)))
        }

        // Γ |- t₁ : T₁₁ -> T₁₂    Γ |- t₂ : T₁₁
        // ------------------------------------- T-App
        //           Γ |- t₁ t₂ : T₁₂
        | Term::App { fun, arg } => {
            let fun_type = fun.check(context, depth)?;
            let arg_type = arg.check(context, depth)?;
            match fun_type {
            | Type::Fun(expected_arg_type, return_type) if *expected_arg_type == arg_type => Ok(*return_type),
            | Type::Fun(_, _) => Err(anyhow!("Parameter type mismatch")),
            | _ => Err(anyhow!("Function type expected")),
            }
        }

        //   Γ |- t₁ : T
        // ---------------- T-Ascribe
        // Γ |- t₁ as T : T
        | Term::Asc { term, r#type: expected_type } => {
            let actual_type = term.check(context, depth)?;
            if actual_type == *expected_type {
                Ok(actual_type)
            } else {
                Err(anyhow!("Expected type {}, but found type {}", expected_type, actual_type))
            }
        }

        // Γ |- t₁ : T₁    Γ, x : T₁ |- t₂ : T₂
        // ------------------------------------ T-Let
        //     Γ |- let x = t₁ in t₂ : T₂
        | Term::Let { hint: _, arg, body } => {
            let arg_type = arg.check(context, depth)?;
            context.0.insert(depth, arg_type);
            let body_type = body.check(context, depth + 1)?;
            context.0.remove(&depth);
            Ok(body_type)
        }

        //         for each i, Γ |- t_i : T_i
        // ---------------------------------------- T-Tuple
        // Γ |- (t_i^{i ∈ 0..n}) : (T_i^{i ∈ 0..n})
        | Term::Tuple(terms) => {
            terms
                .iter()
                .map(|term| term.check(context, depth))
                .collect::<Result<Vec<_>, _>>()
                .map(Type::Tuple)
        }

        // Γ |- t₁ : (T_i^{i ∈ 0..n})
        // -------------------------- T-Proj
        //      Γ |- t₁.j : T_j
        | Term::TupleProject { tuple, index } => {
            match tuple.check(context, depth)? {
            | Type::Tuple(mut types) if *index < types.len() => {
                Ok(types.swap_remove(*index))
            }
            | Type::Tuple(_) => Err(anyhow!("Projecting more elements than tuple contains")),
            | _ => Err(anyhow!("Can only project tuples")),
            }
        }

        //         for each i, Γ |- t_i : T_i
        // ---------------------------------------- T-Rcd
        // Γ |- {l_i=t_i^{i ∈ 0..n}} : {l_i=T_i^{i ∈ 0..n}}
        | Term::Record(terms) => {
            terms
                .iter()
                .map(|(label, term)| {
                    term.check(context, depth)
                        .map(|term| (label.to_owned(), term))
                })
                .collect::<Result<IndexMap<_, _>, _>>()
                .map(Type::Record)
        }
        // Γ |- t₁ : {l_i=T_i^{i ∈ 0..n}}
        // -------------------------- T-Proj
        //      Γ |- t₁.l_j : T_j
        | Term::RecordProject { record, label } => {
            match record.check(context, depth)? {
            | Type::Record(mut types) if types.contains_key(label) => {
                Ok(types.remove(label).unwrap())
            }
            | Type::Record(_) => Err(anyhow!("Projecting unknown label from record")),
            | _ => Err(anyhow!("Can only project records or tuples")),
            }
        }
        }
    }
}
