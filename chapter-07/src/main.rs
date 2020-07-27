use std::io;
use std::io::Write as _;

use typed_arena::Arena;

use chapter_07::term::Context;
use chapter_07::term::Term;

pub fn main() -> anyhow::Result<()> {

    let arena = Arena::new();

    let terms = vec![
        // (λx. x) (λy. y) -->* λy. y
        Term::App {
            fun: arena.alloc(Term::Abs {
                hint: String::from("x"),
                term: arena.alloc(Term::Var { index: 0 }),
            }),
            arg: arena.alloc(Term::Abs {
                hint: String::from("y"),
                term: arena.alloc(Term::Var { index: 0 }),
            }),
        },

        // (λx. λx. x) (λa. λb. a) (λa. λb. b) -->* λa. λb. b
        Term::App {
            fun: arena.alloc(Term::App {
                fun: arena.alloc(Term::Abs {
                    hint: String::from("x"),
                    term: arena.alloc(Term::Abs {
                        hint: String::from("x"),
                        term: arena.alloc(Term::Var { index: 0 }),
                    })
                }),
                arg: arena.alloc(Term::Abs {
                    hint: String::from("a"),
                    term: arena.alloc(Term::Abs {
                        hint: String::from("b"),
                        term: arena.alloc(Term::Var { index: 1 }),
                    })
                }),
            }),
            arg: arena.alloc(Term::Abs {
                hint: String::from("a"),
                term: arena.alloc(Term::Abs {
                    hint: String::from("b"),
                    term: arena.alloc(Term::Var { index: 0 }),
                })
            }),
        },

        // (λx. λz. x) (λz. z) -->* λz. λz'. z'
        Term::App {
            fun: arena.alloc(Term::Abs {
                hint: String::from("x"),
                term: arena.alloc(Term::Abs {
                    hint: String::from("z"),
                    term: arena.alloc(Term::Var { index: 1 }),
                }),
            }),
            arg: arena.alloc(Term::Abs {
                hint: String::from("z"),
                term: arena.alloc(Term::Var { index: 0 }),
            }),
        },
    ];

    for term in terms {
        step(term, &arena)?;
    }

    Ok(())
}

fn step<'a>(
    mut term: Term<'a>,
    arena: &'a Arena<Term<'a>>,
) -> anyhow::Result<()> {

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut context = Context::default();

    term.write(&mut context, &mut stdout)?;
    write!(&mut stdout, "\n")?;

    while let Some(next) = term.step(&arena) {
        term = next;
        term.write(&mut context, &mut stdout)?;
        write!(&mut stdout, "\n")?;
    }

    write!(&mut stdout, "\n")?;
    Ok(())
}
