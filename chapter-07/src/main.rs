use std::io;
use std::io::Write as _;

use typed_arena::Arena;

use chapter_07::term::Context;
use chapter_07::term::Term;

pub fn main() -> anyhow::Result<()> {

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let arena = Arena::new();
    let mut context = Context::default();

    let mut term = Term::App {
        fun: arena.alloc(Term::Abs {
            hint: String::from("x"),
            term: arena.alloc(Term::Var { index: 0 }),
        }),
        arg: arena.alloc(Term::Abs {
            hint: String::from("y"),
            term: arena.alloc(Term::Var { index: 0 }),
        }),
    };

    term.write(&mut context, &mut stdout)?;
    write!(&mut stdout, "\n")?;

    while let Some(next) = term.step(&arena) {
        term = next;
        term.write(&mut context, &mut stdout)?;
        write!(&mut stdout, "\n")?;
    }

    Ok(())
}
