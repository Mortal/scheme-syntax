use std::io;
use std::io::{stdin, Read, Bytes};

mod lexer;
use lexer::IteratorLexer;
mod parser;
use parser::{Node, Parser};

struct CharsWrap<'a, R> where R: Read {
    c: Bytes<R>,
    e: &'a mut Option<io::Error>,
}

impl <'a, R> CharsWrap<'a, R> where R: Read {
    fn new(c: Bytes<R>, e: &'a mut Option<io::Error>) -> Self {
        CharsWrap {
            c: c,
            e: e,
        }
    }
}

impl <'a, R> Iterator for CharsWrap<'a, R> where R: Read {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self.c.next() {
            Some(Ok(a)) => Some(a as char),
            Some(Err(e)) => {*self.e = Some(e); None},
            None => None,
        }
    }
}

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
