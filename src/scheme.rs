use parser::Node;

pub mod syntax {
    pub use lexer::Literal;
    #[derive(Debug)]
    pub enum Quotation {
        Literal(Literal),
        Symbol(String),
        Nil,
        Cons(Box<Quotation>, Box<Quotation>),
    }
    #[derive(Debug)]
    pub enum Expression {
        Literal(Literal),
        Variable(String),
        Quote(Quotation),
    }
}

use std::num::ParseIntError;
use scheme::syntax::{Literal, Quotation, Expression};

#[derive(Debug)]
pub enum SchemeError {
    ParseIntError(ParseIntError),
    Basic(String),
}

impl std::fmt::Display for SchemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &SchemeError::ParseIntError(ref e) => e.fmt(f),
            &SchemeError::Basic(ref s) => write!(f, "SchemeError: {}", s),
        }
    }
}

use std;
pub type Result<T> = std::result::Result<T, SchemeError>;

fn parse_number_literal(s: &str) -> Result<Literal> {
    s.parse::<i32>().map(Literal::Number)
    .map_err(SchemeError::ParseIntError)
}

fn parse_boolean_literal(s: &str) -> Result<Literal> {
    Ok(Literal::Boolean(
        if s == "#t" {true} else if s == "#f" {false}
        else {return Err(SchemeError::Basic("Not a boolean".to_string()));}
    ))
}

fn parse_character_literal(s: &str) -> Result<Literal> {
    Err(SchemeError::Basic("Not implemented".to_string()))
}

fn parse_string_literal(s: &str) -> Result<Literal> {
    Err(SchemeError::Basic("Not implemented".to_string()))
}

fn parse_literal(s: &str) -> Result<Literal> {
    parse_number_literal(s)
    .or(parse_boolean_literal(s))
    .or(parse_character_literal(s))
    .or(parse_string_literal(s))
    .or(Err(SchemeError::Basic("invalid literal".to_string())))
}

fn parse_literal_expression(s: &str) -> Result<Expression> {
    parse_literal(s).map(Expression::Literal)
}

fn parse_variable(s: String) -> Result<Expression> {
    Ok(Expression::Variable(s))
}

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

#[cfg(test)]
mod tests {
    use super::parse_literal;
    use super::syntax::*;

    #[test]
    fn parse_literal_1() {
        assert_eq!(parse_literal("42").unwrap(), Literal::Number(42));
    }
    #[test]
    fn parse_literal_2() {
        assert_eq!(parse_literal("#t").unwrap(), Literal::Boolean(true));
    }
}
