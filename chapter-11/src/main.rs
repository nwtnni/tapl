use std::io;
use std::io::Write as _;

use typed_arena::Arena;

use chapter_11::term::Context;
use chapter_11::term::Term;

pub fn main() -> anyhow::Result<()> {

    let arena = Arena::new();

    let terms = vec![];

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
