use std::error;
use std::str;

use crate::ast;

#[derive(pest_derive::Parser)]
#[grammar = "grammar.pest"]
struct ArithmeticParser;

fn process(pair: pest::iterators::Pair<Rule>) -> ast::Exp {

    let rule = pair.as_rule();
    let mut iter = pair.into_inner();

    macro_rules! go {
        () => {
            iter.next().map(process).map(Box::new).unwrap()
        }
    }

    match rule {
    | Rule::Cond => ast::Exp::Cond(go!(), go!(), go!()),
    | Rule::Succ => ast::Exp::Succ(go!()),
    | Rule::Pred => ast::Exp::Pred(go!()),
    | Rule::IsZero => ast::Exp::IsZero(go!()),
    | Rule::Zero => ast::Exp::Zero,
    | Rule::True => ast::Exp::True,
    | Rule::False => ast::Exp::False,
    | _ => unreachable!(),
    }
}

impl str::FromStr for ast::Exp {
    type Err = Box<dyn error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use pest::Parser;
        let program = ArithmeticParser::parse(Rule::Program, s)?
            .next()
            .unwrap();
        Ok(process(program))
    }
}
