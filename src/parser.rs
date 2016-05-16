use lexer::{Lexer, Token};

#[derive(Debug)]
pub enum Node {
    Atom(String),
    List(Vec<Node>),
}

pub fn parse_next<L>(lexer: &mut L) -> Option<Result<Node, &'static str>>
where L: Lexer {
    let mut stack = vec![];
    while let Some(token_result) = lexer.next() {
        let tok = token_result.unwrap_or_else(
            |e| return Some(Err("lexer error")));
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
            Token::Atom(s) => match stack.last_mut() {
                None => return Some(Ok(Node::Atom(s))),
                Some(m) => m.push(Node::Atom(s)),
            },
        }
    }
    None
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
