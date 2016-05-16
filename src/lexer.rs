use std;
extern crate regex;
use self::regex::{Regex, FindCaptures};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(i32),
    Boolean(bool),
    Character(char),
    String(String),
}

#[derive(Debug)]
pub enum Token {
    Identifier(String),
    LParen,
    RParen,
    Literal(Literal),
}

pub type Result<T> = std::result::Result<T, &'static str>;

pub trait Lexer : Iterator<Item=Result<Token>> {}
impl <I> Lexer for I where I: Iterator<Item=Result<Token>> {}

pub struct IteratorLexer<I> where I: Iterator<Item=char> {
    chars: I,
    peekbuf: Option<char>,
}

impl <I> IteratorLexer<I> where I: Iterator<Item=char> {
    pub fn new(mut it: I) -> Self {
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
            Some('[') => Some(Token::LParen),
            Some(']') => Some(Token::RParen),
            _ => None,
        }
    }
}

impl <I> Iterator for IteratorLexer<I> where I: Iterator<Item=char> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        self.skipws();
        if let Some(t) = self.peeksingle() {
            self.skip();
            return Some(Ok(t));
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
        Some(Ok(Token::Identifier(s)))
    }
}

pub struct RegexLexer<'t> {
    lexer_re: Regex,
    rest: &'t str,
}

impl <'t> RegexLexer<'t> {
    pub fn new(text: &'t str) -> Self {
        let lexer_re = Regex::new(
            r#"(?xi)[ \n]*
               (?P<lparen>\()|
               (?P<rparen>\))|
               (?P<identifier>[a-z!$%&*/:<=>?~_^]
                    [a-z!$%&*/:<=>?~_^0-9.+-]*)|
               (?P<boolean>#[tf])|
               (?P<number>[0-9]+)|
               (?P<character>#\\(?:newline|space|.))|
               (?P<string>"(?:[^\\]|\\.)*")"#).unwrap();

        RegexLexer {
            lexer_re: lexer_re,
            rest: text,
        }
    }
}

impl <'t> Iterator for RegexLexer<'t> {
    type Item = Result<Token>;

    fn next(&mut self) -> Option<Result<Token>> {
        let mo = match self.lexer_re.captures(self.rest) {
            None => return None,
            Some(mo) => mo,
        };
        let (begin, end) = mo.pos(0).unwrap();
        if begin != 0 {
            return Some(Err("unmatched"));
        }
        self.rest = &self.rest[end..];
        let (groupname, value) = mo.iter_named().filter_map(
            |(key, value_opt)|
            match value_opt {
                Some(value) => Some((key, value)),
                None => None,
            }).next().unwrap();

        fn parse_literal(groupname: &str, value: &str) -> Literal {
            if groupname == "number" {
                Literal::Number(value.parse::<i32>().unwrap())
            } else if groupname == "boolean" {
                if value == "#t" { Literal::Boolean(true) }
                else if value == "#f" { Literal::Boolean(false) }
                else { panic!("unknown boolean {}", value) }
            } else if groupname == "character" {
                if value == "#\\newline" { Literal::Character('\n') }
                else if value == "#\\space" { Literal::Character(' ') }
                else { Literal::Character(value.chars().nth(2).unwrap()) }
            } else if groupname == "string" {
                Literal::String(value.to_string())
            } else { panic!("unknown match group {}", groupname) }
        }

        Some(Ok(
            if groupname == "lparen" { Token::LParen }
            else if groupname == "rparen" { Token::RParen }
            else { Token::Literal(parse_literal(groupname, value)) }))
    }
}
