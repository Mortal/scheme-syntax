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

trait Lexer : Iterator<Item=Token> {}
impl <I> Lexer for I where I: Iterator<Item=Token> {}

struct IteratorLexer<I> where I: Iterator<Item=char> {
    chars: I,
    peekbuf: Option<char>,
}

impl <I> IteratorLexer<I> where I: Iterator<Item=char> {
    fn new(mut it: I) -> Self {
        let c = it.next();
        IteratorLexer {
            chars: it,
            peekbuf: c,
        }
    }

    fn skip(&mut self) -> Option<char> {
        let c = self.peekbuf;
        self.peekbuf = self.chars.next();
        c
    }

    fn peekws(&self) -> bool {
        self.peekbuf == Some(' ') || self.peekbuf == Some('\n')
    }

    fn skipws(&mut self) {
        while self.peekws() {
            self.skip();
        }
    }

    fn peeksingle(&mut self) -> Option<Token> {
        match self.peekbuf {
            Some('(') => Some(Token::LParen),
            Some(')') => Some(Token::RParen),
            _ => None,
        }
    }
}

impl <I> Iterator for IteratorLexer<I> where I: Iterator<Item=char> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        self.skipws();
        if let Some(t) = self.peeksingle() {
            self.skip();
            return Some(t);
        }
        let c = match self.skip() {
            None => return None,
            Some(c) => c,
        };

        let mut s = String::new();
        s.push(c);
        loop {
            if let Some(_) = self.peeksingle() { break; }
            if self.peekws() { break; }
            match self.skip() {
                Some(c) => s.push(c),
                None => break,
            };
        }
        Some(Token::Symbol(s))
    }
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
