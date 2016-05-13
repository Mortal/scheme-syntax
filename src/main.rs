use std::io;
use std::io::{stdin, Read, Bytes};

#[derive(Debug)]
enum Token {
    Symbol(String),
    LParen,
    RParen,
}

#[derive(Debug)]
enum Node {
    Symbol(String),
    List(Vec<Node>),
}

struct Lexer<I> where I: Iterator<Item=char> {
    chars: I,
    peekbuf: Option<char>,
}

impl <I> Lexer<I> where I: Iterator<Item=char> {
    fn new(mut it: I) -> Self {
        let c = it.next();
        Lexer {
            chars: it,
            peekbuf: c,
        }
    }

    fn skip(&mut self) -> Option<char> {
        let c = self.peekbuf;
        self.peekbuf = self.chars.next();
        c
    }
}

impl <I> Iterator for Lexer<I> where I: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        match self.skip() {
            Some(' ') => self.next(),
            Some('\n') => self.next(),
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            None => None,
            Some(c) => {
                let mut s = String::new();
                s.push(c);
                loop {
                    match self.peekbuf {
                        Some('(') => break,
                        Some(')') => break,
                        None => break,
                        Some(' ') => break,
                        Some('\n') => break,
                        Some(c) => {
                            s.push(c);
                            self.skip();
                        }
                    }
                }
                Some(Token::Symbol(s))
            },
        }
    }
}

struct Parser<I> where I: Iterator<Item=char> {
    lexer: Lexer<I>,
}

impl <I> Parser<I> where I: Iterator<Item=char> {
    fn new(it: I) -> Self {
        Parser {
            lexer: Lexer::new(it),
        }
    }
}

impl <I> Iterator for Parser<I> where I: Iterator<Item=char> {
    type Item = Result<Node, &'static str>;

    fn next(&mut self) -> Option<Result<Node, &'static str>> {
        let mut stack = vec![];
        while let Some(tok) = self.lexer.next() {
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
    let c = CharsWrap::new(stdin().bytes(), &mut e);
    let parser = Parser::new(c);
    for node in parser {
        match node {
            Ok(node) => println!("{:?}", node),
            Err(s) => println!("error: {}", s),
        }
    }
}

fn parse(s: &str) -> Result<Node, &'static str> {
    Parser::new(s.chars()).next().unwrap_or(Err("None"))
}

#[test]
fn parse_test() {
    parse("(foo bar)").unwrap();
    parse("foo").unwrap();
    parse("foo )garbage").unwrap();
    parse(")").unwrap_err();
    parse("").unwrap_err();
}
