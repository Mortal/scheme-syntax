use std::io;
use std::io::{stdin, Read, Bytes};

mod lexer;
use lexer::{Token, Lexer, IteratorLexer};

#[derive(Debug)]
enum Node {
    Symbol(String),
    List(Vec<Node>),
}

fn parse_next<L>(lexer: &mut L) -> Option<Result<Node, &'static str>> where L: Lexer {
    let mut stack = vec![];
    while let Some(tok) = lexer.next() {
        match tok {
            Token::LParen => stack.push(vec![]),
            Token::RParen => {
                let c = match stack.pop() {
                    Some(c) => c,
                    None => return Some(Err("unmatched right parenthesis")),
                };
                match stack.last_mut() {
                    None => return Some(Ok(Node::List(c))),
                    Some(m) => m.push(Node::List(c)),
                }
            },
            Token::Symbol(s) => match stack.last_mut() {
                None => return Some(Ok(Node::Symbol(s))),
                Some(m) => m.push(Node::Symbol(s)),
            },
        }
    }
    None
}

struct Parser<L> where L: Lexer {
    lexer: L,
}

impl <L> Parser<L> where L: Lexer {
    fn new(lexer: L) -> Self {
        Parser {
            lexer: lexer,
        }
    }
}

impl <L> Iterator for Parser<L> where L: Lexer {
    type Item = Result<Node, &'static str>;

    fn next(&mut self) -> Option<Result<Node, &'static str>> {
        parse_next(&mut self.lexer)
    }
}

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
