#[derive(Debug)]
pub enum Token {
    Symbol(String),
    LParen,
    RParen,
}

pub trait Lexer : Iterator<Item=Token> {}
impl <I> Lexer for I where I: Iterator<Item=Token> {}

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
