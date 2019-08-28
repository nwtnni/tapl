#![feature(box_patterns)]
#![feature(bind_by_move_pattern_guards)]

use std::env;

mod ast;
mod eval;
mod parse;
mod display;

fn main() {
    let arg = env::args().collect::<Vec<_>>();
    unimplemented!()
}
