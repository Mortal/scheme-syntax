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
    pub enum CondClause {
        Simple(Expression, Expression),
        Binding(Expression, Expression),
        Inconsequential(Expression),
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
        Cond(Vec<CondClause>, Box<Expression>),
    }
}

use scheme::syntax::{Quotation, Expression, CondClause};

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

fn unary_op<C>(ctor: C, mut tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>) -> Expression {
    if tl.len() != 1 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 1, got {}", tl.len())));
    }
    let arg1 = Box::new(try!(parse_expression(tl.pop().unwrap())));
    Ok(ctor(arg1))
}

fn binary_op<C>(ctor: C, mut tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>, Box<Expression>) -> Expression {
    if tl.len() != 2 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 2, got {}", tl.len())));
    }
    let arg2 = Box::new(try!(parse_expression(tl.pop().unwrap())));
    let arg1 = Box::new(try!(parse_expression(tl.pop().unwrap())));
    Ok(ctor(arg1, arg2))
}

fn ternary_op<C>(ctor: C, mut tl: Vec<Node>) -> Result<Expression>
where C: FnOnce(Box<Expression>, Box<Expression>, Box<Expression>) -> Expression {
    if tl.len() != 3 {
        return Err(SchemeError::Basic(
            format!("Wrong number of arguments: expected 3, got {}", tl.len())));
    }
    let arg3 = Box::new(try!(parse_expression(tl.pop().unwrap())));
    let arg2 = Box::new(try!(parse_expression(tl.pop().unwrap())));
    let arg1 = Box::new(try!(parse_expression(tl.pop().unwrap())));
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

fn parse_cond_clause_inconsequential(test: Node) -> Result<CondClause> {
    Ok(CondClause::Inconsequential(try!(parse_expression(test))))
}

fn parse_cond_clause_simple(test: Node, consequent: Node) -> Result<CondClause> {
    Ok(CondClause::Simple(
        try!(parse_expression(test)),
        try!(parse_expression(consequent))))
}

fn parse_cond_clause_binding(test: Node, arrow: Node, consequent: Node) -> Result<CondClause> {
    match arrow {
        Node::Identifier(a) =>
            if a != "=>" {
                return Err(SchemeError::Basic(
                    "cond clause: middle argument must be =>".to_string()));
            },
        _ => return Err(SchemeError::Basic(
            "cond clause: middle argument must be =>".to_string())),
    };
    Ok(CondClause::Binding(
        try!(parse_expression(test)),
        try!(parse_expression(consequent))))
}

fn parse_cond_clause(clause: Node) -> Result<CondClause> {
    let mut l = match clause {
        Node::List(l) => l,
        _ => return Err(SchemeError::Basic(
            "cond clause: Expected list".to_string())),
    };
    if l.len() == 1 {
        parse_cond_clause_inconsequential(l.pop().unwrap())
    } else if l.len() == 2 {
        let a2 = l.pop().unwrap();
        let a1 = l.pop().unwrap();
        parse_cond_clause_simple(a1, a2)
    } else if l.len() == 3 {
        let a3 = l.pop().unwrap();
        let a2 = l.pop().unwrap();
        let a1 = l.pop().unwrap();
        parse_cond_clause_binding(a1, a2, a3)
    } else {
        Err(SchemeError::Basic(
            format!("cond clause: Expected 1 <= length <= 3, got {}", l.len())))
    }
}

fn get_cond_else(else_clause: Node) -> Result<Node> {
    let mut l = match else_clause {
        Node::List(l) => l,
        _ => return Err(SchemeError::Basic(
            "cond clause: Expected list".to_string())),
    };
    if l.len() != 2 {
        return Err(SchemeError::Basic(
            format!("cond else clause: Expected length 2, got {}", l.len())));
    }
    let a2 = l.pop().unwrap();
    let a1 = l.pop().unwrap();
    let else_id = match a1 {
        Node::Identifier(id) => id,
        _ => return Err(SchemeError::Basic(
            "cond else clause: Expected else".to_string())),
    };
    if else_id != "else" {
        return Err(SchemeError::Basic(
            "cond else clause: Expected else".to_string()));
    }
    Ok(a2)
}

fn parse_cond(mut clauses: Vec<Node>) -> Result<Expression> {
    let n = clauses.len();
    if n == 0 {
        return Err(SchemeError::Basic(
            "Wrong number of cond arguments: expected at least 1, got 0"
            .to_string()));
    }
    let else_clause = clauses.split_off(n-1).into_iter().next().unwrap();
    let mut res = Vec::new();
    for c in clauses.into_iter() {
        res.push(try!(parse_cond_clause(c)));
    }
    let else_clause = try!(parse_expression(
        try!(get_cond_else(else_clause))));
    Ok(Expression::Cond(res, Box::new(else_clause)))
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
            } else if keyword == "cond" {
                parse_cond(tl)
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
