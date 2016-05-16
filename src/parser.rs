use lexer::{Lexer, Token, Literal};

#[derive(Debug)]
pub enum Node {
    Identifier(String),
    Literal(Literal),
    List(Vec<Node>),
}

pub fn parse_next<L>(lexer: &mut L) -> Option<Result<Node, &'static str>>
where L: Lexer {
    let mut stack = vec![];
    while let Some(token_result) = lexer.next() {
        let tok = match token_result {
            Ok(tok) => tok,
            Err(_) => return Some(Err("lexer error")),
        };
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
            Token::Identifier(s) => match stack.last_mut() {
                None => return Some(Ok(Node::Identifier(s))),
                Some(m) => m.push(Node::Identifier(s)),
            },
            Token::Literal(l) => match stack.last_mut() {
                None => return Some(Ok(Node::Literal(l))),
                Some(m) => m.push(Node::Literal(l)),
            },
        }
    }
    if stack.len() == 0 {
        None
    } else {
        Some(Err("unexpected EOF"))
    }
}

pub struct Parser<L> where L: Lexer {
    lexer: L,
}

impl <L> Parser<L> where L: Lexer {
    pub fn new(lexer: L) -> Self {
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
