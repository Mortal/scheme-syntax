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
        Time(Box<Expression>),
        If(Box<Expression>, Box<Expression>, Box<Expression>),
        And(Vec<Expression>),
        Or(Vec<Expression>),
        Begin(Vec<Expression>),
        Unless(Box<Expression>, Box<Expression>),
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

fn parse_quotation(e: Node) -> Result<Quotation> {
    match e {
        Node::Identifier(s) => Ok(Quotation::Symbol(s.clone())),
        Node::Literal(l) => Ok(Quotation::Literal(l.clone())),
        Node::List(s) => parse_quotation_list(s),
    }
}

fn parse_quotation_list(mut e: Vec<Node>) -> Result<Quotation> {
    if e.len() == 0 { return Ok(Quotation::Nil); }
    let tl = e.split_off(1);
    let hd = e.pop().unwrap();
    Ok(Quotation::Cons(
        Box::new(try!(parse_quotation(hd))),
        Box::new(try!(parse_quotation_list(tl)))))
}

fn unary_op<C>(ctor: C, tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>) -> Expression {
    if tl.len() != 1 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 1, got {}", tl.len())));
    }
    let mut tl = tl.into_iter();
    let arg1 = Box::new(try!(parse_expression(tl.next().unwrap())));
    Ok(ctor(arg1))
}

fn binary_op<C>(ctor: C, tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>, Box<Expression>) -> Expression {
    if tl.len() != 2 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 2, got {}", tl.len())));
    }
    let mut tl = tl.into_iter();
    let arg1 = Box::new(try!(parse_expression(tl.next().unwrap())));
    let arg2 = Box::new(try!(parse_expression(tl.next().unwrap())));
    Ok(ctor(arg1, arg2))
}

fn ternary_op<C>(ctor: C, tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>, Box<Expression>, Box<Expression>) -> Expression {
    if tl.len() != 3 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 3, got {}", tl.len())));
    }
    let mut tl = tl.into_iter();
    let arg1 = Box::new(try!(parse_expression(tl.next().unwrap())));
    let arg2 = Box::new(try!(parse_expression(tl.next().unwrap())));
    let arg3 = Box::new(try!(parse_expression(tl.next().unwrap())));
    Ok(ctor(arg1, arg2, arg3))
}

fn zero_or_more_op<C>(ctor: C, tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Vec<Expression>) -> Expression {
    let mut args = Vec::new();
    for n in tl.into_iter() {
        args.push(try!(parse_expression(n)));
    }
    Ok(ctor(args))
}

fn one_or_more_op<C>(ctor: C, tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Vec<Expression>) -> Expression {
    if tl.len() == 0 {
        return Err(SchemeError::Basic(
            "Wrong number of arguments: expected at least 1, got 0".to_string()));
    }
    zero_or_more_op(ctor, tl)
}

fn parse_expression_from_list(hd: Node, tl: Vec<Node>) -> Result<Expression> {
    match hd {
        Node::Identifier(ref keyword) =>
            if keyword == "quote" {
                Ok(Expression::Quote(try!(parse_quotation_list(tl))))
            } else if keyword == "time" {
                unary_op(Expression::Time, tl)
            } else if keyword == "if" {
                ternary_op(Expression::If, tl)
            } else if keyword == "and" {
                zero_or_more_op(Expression::And, tl)
            } else if keyword == "or" {
                zero_or_more_op(Expression::Or, tl)
            } else if keyword == "begin" {
                one_or_more_op(Expression::Begin, tl)
            } else if keyword == "unless" {
                binary_op(Expression::Unless, tl)
            } else {
                Err(SchemeError::Basic(format!("unhandled keyword {}", keyword)))
            },
        Node::Literal(_) =>
            Err(SchemeError::Basic("Cannot apply to literal".to_string())),
        Node::List(_) =>
            Err(SchemeError::Basic("Application not implemented".to_string())),
    }
}

pub fn parse_expression(n: Node) -> Result<Expression> {
    match n {
        Node::Literal(l) => Ok(Expression::Literal(l)),
        Node::Identifier(s) => Ok(Expression::Variable(s)), // TODO check reserved
        Node::List(mut s) => {
            if s.len() == 0 {
                return Err(SchemeError::Basic("Unexpected Nil".to_string()));
            }
            let tl = s.split_off(1);
            let hd = s.pop().unwrap();
            parse_expression_from_list(hd, tl)
        },
    }
}
