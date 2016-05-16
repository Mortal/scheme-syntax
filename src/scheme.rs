use parser::Node;

pub mod syntax {
    pub use lexer::Literal;
    #[derive(Debug, PartialEq)]
    pub enum Quotation {
        Literal(Literal),
        Symbol(String),
        Nil,
        Cons(Box<Quotation>, Box<Quotation>),
    }
    #[derive(Debug, PartialEq)]
    pub enum Expression {
        Literal(Literal),
        Variable(String),
        Quote(Quotation),
    }
}

use scheme::syntax::{Quotation, Expression};

#[derive(Debug)]
pub enum SchemeError {
    Basic(String),
}

impl std::fmt::Display for SchemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &SchemeError::Basic(ref s) => write!(f, "SchemeError: {}", s),
        }
    }
}

use std;
pub type Result<T> = std::result::Result<T, SchemeError>;

fn parse_quotation(e: &Node) -> Result<Quotation> {
    match e {
        &Node::Identifier(ref s) => Ok(Quotation::Symbol(s.clone())),
        &Node::Literal(ref l) => Ok(Quotation::Literal(l.clone())),
        &Node::List(ref s) => parse_quotation_list(&s[..]),
    }
}

fn parse_quotation_list(e: &[Node]) -> Result<Quotation> {
    match e.split_first() {
        None => Ok(Quotation::Nil),
        Some((hd, tl)) => Ok(Quotation::Cons(
            Box::new(try!(parse_quotation(hd))), Box::new(try!(parse_quotation_list(tl))))),
    }
}

fn parse_expression_from_list(hd: &Node, tl: &[Node]) -> Result<Expression> {
    match hd {
        &Node::Identifier(ref keyword) =>
            if keyword == "quote" {
                Ok(Expression::Quote(try!(parse_quotation_list(tl))))
            } else {
                Err(SchemeError::Basic(format!("unhandled keyword {}", keyword)))
            },
        &Node::Literal(_) =>
            Err(SchemeError::Basic("Cannot apply to literal".to_string())),
        &Node::List(_) =>
            Err(SchemeError::Basic("Application not implemented".to_string())),
    }
}

pub fn parse_expression(n: Node) -> Result<Expression> {
    match n {
        Node::Literal(l) => Ok(Expression::Literal(l)),
        Node::Identifier(s) => Ok(Expression::Variable(s)), // TODO check reserved
        Node::List(s) =>
            match s.split_first() {
                None => Err(SchemeError::Basic("Unexpected Nil".to_string())),
                Some((hd, tl)) => parse_expression_from_list(hd, tl),
            },
    }
}
