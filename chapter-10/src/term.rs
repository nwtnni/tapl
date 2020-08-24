use std::io;
use std::iter;

use typed_arena::Arena;

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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Term<'a> {
    Var {
        /// de Bruijn index
        index: i64,
    },
    Abs {
        /// Hint for the name of the bound variable
        hint: String,
        term: &'a Term<'a>,
    },
    App {
        fun: &'a Term<'a>,
        arg: &'a Term<'a>,
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
        | Term::App { fun: Term::Abs { term, .. }, arg } if arg.is_value() => {
            Some(term.substitute_top(arena, arg))
        }
        | Term::App { fun, arg } if fun.is_value() => {
            Some(Term::App {
                fun,
                arg: arena.alloc(arg.step(arena)?),
            })
        }
        | Term::App { fun, arg } => {
            Some(Term::App {
                fun: arena.alloc(fun.step(arena)?),
                arg,
            })
        }
        | _ => None,
        }
    }

    pub fn is_value(&self) -> bool {
        match self {
        | Term::Var { .. }
        | Term::Abs { .. } => true,
        | Term::App { .. } => false,
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
        | Term::Var { index } if *index == from + depth => to.shift(arena, depth),
        | Term::Var { index } => Term::Var { index: *index },
        | Term::Abs { hint, term } => {
            Term::Abs {
                hint: hint.to_owned(),
                term: arena.alloc(term._substitute(arena, from, to, depth + 1)),
            }
        }
        | Term::App { fun, arg } => {
            Term::App {
                fun: arena.alloc(fun._substitute(arena, from, to, depth)),
                arg: arena.alloc(arg._substitute(arena, from, to, depth)),
            }
        }
        }
    }

    fn shift(&self, arena: &'a Arena<Term<'a>>, max_depth: i64) -> Self {
        self._shift(arena, max_depth, 0)
    }

    fn _shift(&self, arena: &'a Arena<Term<'a>>, max_depth: i64, depth: i64) -> Self {
        match self {
        | Term::Var { index } if *index >= depth => Term::Var { index: index + max_depth },
        | Term::Var { index } => Term::Var { index: *index },
        | Term::Abs { hint, term } => {
            Term::Abs {
                hint: hint.to_owned(),
                term: arena.alloc(term._shift(arena, max_depth, depth + 1)),
            }
        }
        | Term::App { fun, arg } => {
            Term::App {
                fun: arena.alloc(fun._shift(arena, max_depth, depth)),
                arg: arena.alloc(arg._shift(arena, max_depth, depth)),
            }
        }
        }
    }

    pub fn write<W: io::Write>(&self, context: &mut Context, writer: &mut W) -> anyhow::Result<()> {
        match self {
        | Term::Var { index } => {
            write!(writer, "{}", context.name(*index))?;
        }
        | Term::Abs { hint, term } => {
            let name = context.push(hint.to_owned());
            write!(writer, "(λ{}. ", name)?;
            term.write(context, writer)?;
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
        }
        Ok(())
    }
}
