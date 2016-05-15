use std::io::{stdin, Read};

mod lexer;
use lexer::IteratorLexer;
mod parser;
use parser::{Node, Parser};
mod io;
use io::CharsWrap;

fn main() {
    let mut e = None;
    let chars = CharsWrap::new(stdin().bytes(), &mut e);
    let lexer = IteratorLexer::new(chars);
    let parser = Parser::new(lexer);
    for node in parser {
        match node {
            Ok(node) => println!("{:?}", node),
            Err(s) => println!("error: {}", s),
        }
    }
}

fn parse(s: &str) -> Result<Node, &'static str> {
    let chars = s.chars();
    let lexer = IteratorLexer::new(chars);
    let mut parser = Parser::new(lexer);
    parser.next().unwrap_or(Err("None"))
}

#[test]
fn parse_test() {
    parse("(foo bar)").unwrap();
    parse("foo").unwrap();
    parse("foo )garbage").unwrap();
    parse(")").unwrap_err();
    parse("").unwrap_err();
}
