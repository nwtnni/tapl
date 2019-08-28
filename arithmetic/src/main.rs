#![feature(box_patterns)]
#![feature(bind_by_move_pattern_guards)]

use std::env;
use std::error;
use std::fs;

mod ast;
mod eval;
mod parse;
mod display;

fn main() -> Result<(), Box<dyn error::Error>> {
    let path = env::args()
        .nth(1)
        .expect("Usage: cargo run -- <FILE>");
    fs::read_to_string(&path)?
        .parse::<ast::Exp>()?
        .step_all()
        .for_each(|exp| println!("{}", exp));
    Ok(())
}
