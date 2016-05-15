use std::io::{stdin, Read};

mod lexer;
use lexer::IteratorLexer;
mod parser;
use parser::{Node, Parser};
mod io;
use io::CharsWrap;
mod scheme;
use scheme::parse_expression;

fn main() {
    let mut e = None;
    let chars = CharsWrap::new(stdin().bytes(), &mut e);
    let lexer = IteratorLexer::new(chars);
    let parser = Parser::new(lexer);
    for node_result in parser {
        match node_result {
            Err(e) => println!("error: {}", e),
            Ok(node) =>
                match parse_expression(node) {
                    Err(e) => println!("error: {}", e),
                    Ok(expr) => println!("{:?}", expr),
                },
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
