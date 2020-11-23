use std::io;
use std::iter;

use indexmap::IndexMap;
use typed_arena::Arena;

use crate::r#type::Type;

#[derive(Clone, Debug, Default)]
pub struct Context(Vec<String>);

impl Context {
    pub fn len(&self) -> i64 {
        self.0.len() as i64
    }

    pub fn push<'c>(&'c mut self, mut var: String) -> &str {
        while let Some(_) = self.0.iter().find(|entry| **entry == var) {
            var.push('\'');
        }
        self.0.push(var);
        self.0.last().unwrap()
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn name(&self, index: i64) -> &str {
        assert!(index >= 0);
        if index >= self.len() {
            match index - self.len() {
            | 0 => "α",
            | 1 => "β",
            | 2 => "ξ",
            | _ => todo!(),
            }
        } else {
            let index = self.0.len() - 1 - index as usize;
            &self.0[index]
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Term<'a> {
    Bool(bool),
    If {
        r#if: &'a Term<'a>,
        then: &'a Term<'a>,
        r#else: &'a Term<'a>,
    },
    /// de Bruijn index
    Var(i64),
    Abs {
        /// Hint for the name of the bound variable
        hint: String,
        r#type: Type,
        body: &'a Term<'a>,
    },
    App {
        fun: &'a Term<'a>,
        arg: &'a Term<'a>,
    },
    Asc {
        term: &'a Term<'a>,
        r#type: Type,
    },
    Let {
        hint: String,
        arg: &'a Term<'a>,
        body: &'a Term<'a>,
    },
    Tuple(Vec<&'a Term<'a>>),
    TupleProject {
        tuple: &'a Term<'a>,
        index: usize,
    },

    Record(IndexMap<String, &'a Term<'a>>),
    RecordProject {
        record: &'a Term<'a>,
        label: String,
    },
}

impl<'a> Term<'a> {
    pub fn eval(&'a self, arena: &'a Arena<Term<'a>>) -> &'a Self {
        iter::successors(Some(self), |term| {
                term.step(arena)
                    .map(|next| &*arena.alloc(next))
            })
            .last()
            .unwrap_or(self)
    }

    pub fn step(&self, arena: &'a Arena<Term<'a>>) -> Option<Self> {
        match self {
        | Term::Bool(_)
        | Term::Var(_)
        | Term::Abs { .. } => None,

        //
        // ------------------------------- E-AppAbs
        // (λx. t₁₂) v₂ --> [x |-> v₂] t₁₂
        | Term::App { fun: Term::Abs { body, .. }, arg } if arg.is_value() => {
            Some(body.substitute_top(arena, arg))
        }

        //    t₂ --> t₂'
        // ---------------- E-App2
        // v₁ t₂ --> v₁ t₂'
        | Term::App { fun, arg } if fun.is_value() => {
            Some(Term::App {
                fun,
                arg: arena.alloc(arg.step(arena)?),
            })
        }
        //    t₁ --> t₁'
        // ---------------- E-App1
        // t₁ t₂ --> t₁' t₂
        | Term::App { fun, arg } => {
            Some(Term::App {
                fun: arena.alloc(fun.step(arena)?),
                arg,
            })
        }

        //
        // ------------------------------ E-IfTrue
        // if true then t₂ else t₃ --> t₂
        | Term::If { r#if: Term::Bool(true), then, r#else: _ } => Some(Clone::clone(*then)),
        //
        // ------------------------------ E-IfFalse
        // if false then t₂ else t₃ --> t₃
        | Term::If { r#if: Term::Bool(false), then: _, r#else } => Some(Clone::clone(*r#else)),
        //
        //                    t₁ --> t₁'
        // ------------------------------------------------ E-If
        // if t₁ then t₂ else t₃ --> if t₁' then t₂ else t₃
        | Term::If { r#if, then, r#else } => {
            Some(Term::If {
                r#if: arena.alloc(r#if.step(arena)?),
                then,
                r#else,
            })
        }

        //
        // -------------- E-Ascribe
        // v₁ as T --> v₁
        | Term::Asc { term, r#type: _ } if term.is_value() => Some(Clone::clone(*term)),
        //
        //      t₁ --> t₁'
        // -------------------- E-Ascribe1
        // t₁ as T --> t₁' as T
        | Term::Asc { term, r#type } => {
            Some(Term::Asc {
                term: arena.alloc(term.step(arena)?),
                r#type: r#type.clone(),
            })
        }

        //
        // ---------------------------------- E-LetV
        // let x = v₁ in t₂ --> [x |-> v₁] t₂
        | Term::Let { hint: _, arg, body } if arg.is_value() => Some(body.substitute_top(arena, arg)),
        //
        //               t₁ --> t₁'
        // -------------------------------------- E-Let
        // let x = t₁ in t₂ --> let x = t₁' in t₂
        | Term::Let { hint, arg, body } => {
            Some(Term::Let {
                hint: hint.clone(),
                arg: arena.alloc(arg.step(arena)?),
                body,
            })
        }

        //                                         t_j --> t_j'
        // ---------------------------------------------------------------------------------------- E-Tuple
        // (v_i^{i ∈ 0..j-1}, t_j, t_k^{k ∈ j+1..n}) --> (v_i^{i ∈ 0..j-1}, t_j', t_k^{k ∈ j+1..n})
        | Term::Tuple(terms) if terms.iter().any(|term| !term.is_value()) => {
            let mut before = terms.iter();
            let mut after = Vec::new();

            while let Some(&term) = before.next() {
                if term.is_value() {
                    after.push(term);
                } else {
                    after.push(arena.alloc(term.step(arena)?));
                    break;
                }
            }

            after.extend(before);
            Some(Term::Tuple(after))
        }
        | Term::Tuple(_) => None,

        //
        // -------------------------- E-ProjTuple
        // (v_i^{i ∈ i..n}).j --> v_j
        | Term::TupleProject { tuple: Term::Tuple(terms), index } if terms.iter().all(|term| term.is_value()) => {
            Some(terms[*index].clone())
        }
        //   t₁ --> t₁'
        // -------------- E-Proj
        // t₁.i --> t₁'.i
        | Term::TupleProject { tuple: tuple @ Term::Tuple(_), index } => {
            Some(Term::TupleProject {
                tuple: arena.alloc(tuple.step(arena)?),
                index: *index,
            })
        }
        | Term::TupleProject { .. } => panic!("Could not evaluate ill-typed tuple projection term"),

        //                                                   t_j --> t_j'
        // ---------------------------------------------------------------------------------------------------------------- E-Record
        // {l_i=v_i^{i ∈ 0..j-1}, l_j=t_j, l_k=t_k^{k ∈ j+1..n}} --> {l_i=v_i^{i ∈ 0..j-1}, l_j=t_j', l_k=t_k^{k ∈ j+1..n}}
        | Term::Record(terms) if terms.values().any(|term| !term.is_value()) => {
            let mut before = terms.iter().map(|(label, &term)| (label.to_owned(), term));
            let mut after = IndexMap::new();

            while let Some((label, term)) = before.next() {
                if term.is_value() {
                    after.insert(label, term);
                } else {
                    after.insert(label, arena.alloc(term.step(arena)?));
                    break;
                }
            }

            after.extend(before);
            Some(Term::Record(after))
        }
        | Term::Record(_) => None,

        //
        // -------------------------- E-ProjRecord
        // {l_i = v_i^{i ∈ i..n}}.l_j --> v_j
        | Term::RecordProject { record: Term::Record(terms), label } if terms.values().all(|term| term.is_value()) => {
            Some(terms[label].clone())
        }
        //   t₁ --> t₁'
        // -------------- E-Proj
        // t₁.l --> t₁'.l
        | Term::RecordProject { record: record @ Term::Record(_), label } => {
            Some(Term::RecordProject {
                record: arena.alloc(record.step(arena)?),
                label: label.clone(),
            })
        }
        | Term::RecordProject { .. } => panic!("Could not evaluate ill-typed record projection term"),
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
        | Term::Bool(_)
        | Term::Var { .. }
        | Term::Abs { .. } => true,
        | Term::Tuple(terms) => terms.iter().all(|term| term.is_value()),
        | Term::Record(terms) => terms.values().all(|term| term.is_value()),
        | Term::If { .. }
        | Term::App { .. }
        | Term::Asc { .. }
        | Term::Let { .. }
        | Term::TupleProject { .. }
        | Term::RecordProject { .. } => false,
        }
    }

    fn substitute_top(&self, arena: &'a Arena<Term<'a>>, to: &'a Term<'a>) -> Self {
        self.substitute(arena, 0, arena.alloc(to.shift(arena, 1))).shift(arena, -1)
    }

    fn substitute(&self, arena: &'a Arena<Term<'a>>, from: i64, to: &'a Term<'a>) -> Self {
        self._substitute(arena, from, to, 0)
    }

    fn _substitute(
        &self,
        arena: &'a Arena<Term<'a>>,
        from: i64,
        to: &'a Term<'a>,
        depth: i64,
    ) -> Self {
        match self {
        | Term::Bool(bool) => Term::Bool(*bool),
        | Term::If { r#if, then, r#else } => {
            Term::If {
                r#if: arena.alloc(r#if._substitute(arena, from, to, depth)),
                then: arena.alloc(then._substitute(arena, from, to, depth)),
                r#else: arena.alloc(r#else._substitute(arena, from, to, depth)),
            }
        }
        | Term::Var(index) if *index == from + depth => to.shift(arena, depth),
        | Term::Var(index) => Term::Var(*index),
        | Term::Abs { hint, r#type, body } => {
            Term::Abs {
                hint: hint.clone(),
                r#type: r#type.clone(),
                body: arena.alloc(body._substitute(arena, from, to, depth + 1)),
            }
        }
        | Term::App { fun, arg } => {
            Term::App {
                fun: arena.alloc(fun._substitute(arena, from, to, depth)),
                arg: arena.alloc(arg._substitute(arena, from, to, depth)),
            }
        }
        | Term::Asc { term, r#type } => {
            Term::Asc {
                term: arena.alloc(term._substitute(arena, from, to, depth)),
                r#type: r#type.clone(),
            }
        }
        | Term::Let { hint, arg, body } => {
            Term::Let {
                hint: hint.clone(),
                arg: arena.alloc(arg._substitute(arena, from, to, depth)),
                body: arena.alloc(body._substitute(arena, from, to, depth + 1)),
            }
        }
        | Term::Tuple(terms) => {
            Term::Tuple(terms.iter().map(|term| &*arena.alloc(term._substitute(arena, from, to, depth))).collect())
        }
        | Term::TupleProject { tuple, index } => {
            Term::TupleProject {
                tuple: arena.alloc(tuple._substitute(arena, from, to, depth)),
                index: *index,
            }
        }
        | Term::Record(terms) => {
            Term::Record(terms.iter().map(|(label, term)| (
                label.to_owned(),
                &*arena.alloc(term._substitute(arena, from, to, depth)),
            )).collect())
        }
        | Term::RecordProject { record, label } => {
            Term::RecordProject {
                record: arena.alloc(record._substitute(arena, from, to, depth)),
                label: label.to_owned(),
            }
        }
        }
    }

    fn shift(&self, arena: &'a Arena<Term<'a>>, max_depth: i64) -> Self {
        self._shift(arena, max_depth, 0)
    }

    fn _shift(&self, arena: &'a Arena<Term<'a>>, max_depth: i64, depth: i64) -> Self {
        match self {
        | Term::Bool(bool) => Term::Bool(*bool),
        | Term::If { r#if, then, r#else } => {
            Term::If {
                r#if: arena.alloc(r#if._shift(arena, max_depth, depth)),
                then: arena.alloc(then._shift(arena, max_depth, depth)),
                r#else: arena.alloc(r#else._shift(arena, max_depth, depth)),
            }
        }
        | Term::Var(index) if *index >= depth => Term::Var(index + max_depth),
        | Term::Var(index) => Term::Var(*index),
        | Term::Abs { hint, r#type, body } => {
            Term::Abs {
                hint: hint.clone(),
                r#type: r#type.clone(),
                body: arena.alloc(body._shift(arena, max_depth, depth + 1)),
            }
        }
        | Term::App { fun, arg } => {
            Term::App {
                fun: arena.alloc(fun._shift(arena, max_depth, depth)),
                arg: arena.alloc(arg._shift(arena, max_depth, depth)),
            }
        }
        | Term::Asc { term, r#type } => {
            Term::Asc {
                term: arena.alloc(term._shift(arena, max_depth, depth)),
                r#type: r#type.clone(),
            }
        }
        | Term::Let { hint, arg, body } => {
            Term::Let {
                hint: hint.clone(),
                arg: arena.alloc(arg._shift(arena, max_depth, depth)),
                body: arena.alloc(body._shift(arena, max_depth, depth + 1)),
            }
        }
        | Term::Tuple(terms) => {
            Term::Tuple(terms.iter().map(|term| &*arena.alloc(term._shift(arena, max_depth, depth))).collect())
        }
        | Term::TupleProject { tuple, index } => {
            Term::TupleProject {
                tuple: arena.alloc(tuple._shift(arena, max_depth, depth)),
                index: *index,
            }
        }
        | Term::Record(terms) => {
            Term::Record(terms.iter().map(|(label, term)| (
                label.to_owned(),
                &*arena.alloc(term._shift(arena, max_depth, depth)),
            )).collect())
        }
        | Term::RecordProject { record, label } => {
            Term::RecordProject {
                record: arena.alloc(record._shift(arena, max_depth, depth)),
                label: label.to_owned(),
            }
        }
        }
    }

    pub fn write<W: io::Write>(&self, context: &mut Context, writer: &mut W) -> anyhow::Result<()> {
        match self {
        | Term::Bool(bool) => {
            write!(writer, "{}", bool)?;
        }
        | Term::If { r#if, then, r#else } => {
            write!(writer, "if ")?;
            r#if.write(context, writer)?;
            write!(writer, " then ")?;
            then.write(context, writer)?;
            write!(writer, " else ")?;
            r#else.write(context, writer)?;
        }
        | Term::Var(index) => {
            write!(writer, "{}", context.name(*index))?;
        }
        | Term::Abs { hint, r#type, body } => {
            let name = context.push(hint.to_owned());
            write!(writer, "(λ{}: {}. ", r#type, name)?;
            body.write(context, writer)?;
            write!(writer, ")")?;
            context.pop();
        }
        | Term::App { fun, arg } => {
            write!(writer, "(")?;
            fun.write(context, writer)?;
            write!(writer, " ")?;
            arg.write(context, writer)?;
            write!(writer, ")")?;
        }
        | Term::Asc { term, r#type } => {
            term.write(context, writer)?;
            write!(writer, " as {}", r#type)?;
        }
        | Term::Let { hint, arg, body } => {
            let name = context.push(hint.to_owned());
            write!(writer, "let {} = ", name)?;
            arg.write(context, writer)?;
            write!(writer, " in ")?;
            body.write(context, writer)?;
        }
        | Term::Tuple(terms) => {
            let mut terms = terms.iter();
            write!(writer, "(")?;
            if let Some(head) = terms.next() {
                head.write(context, writer)?;
            }
            while let Some(tail) = terms.next() {
                write!(writer, ", ")?;
                tail.write(context, writer)?;
            }
            write!(writer, ")")?;
        }
        | Term::TupleProject { tuple, index } => {
            tuple.write(context, writer)?;
            write!(writer, ".{}", index)?;
        }
        | Term::Record(terms) => {
            let mut terms = terms.iter();
            write!(writer, "{{")?;
            if let Some((label, term)) = terms.next() {
                write!(writer, "{} = ", label)?;
                term.write(context, writer)?;
            }
            while let Some((label, term)) = terms.next() {
                write!(writer, ", {} =", label)?;
                term.write(context, writer)?;
            }
            write!(writer, "}}")?;
        }
        | Term::RecordProject { record, label } => {
            record.write(context, writer)?;
            write!(writer, ".{}", label)?;
        }
        }
        Ok(())
    }
}
