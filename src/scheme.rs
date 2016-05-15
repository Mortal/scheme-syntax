use parser::Node;

pub mod syntax {
    #[derive(Debug)]
    pub enum Literal {
        Number(i32),
        Boolean(bool),
        Character(char),
        String(String),
    }
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

fn parse_literal(s: &str) -> Result<syntax::Literal> {
    s.parse::<i32>()
    .map(|n| syntax::Literal::Number(n))
    .map_err(|e| SchemeError::ParseIntError(e))
}

fn parse_literal_expression(s: &str) -> Result<syntax::Expression> {
    parse_literal(s).map(
        |l| syntax::Expression::Literal(l))
}

fn parse_variable(s: String) -> Result<syntax::Expression> {
    Ok(syntax::Expression::Variable(s))
}

fn parse_quotation(e: &Node) -> Result<syntax::Quotation> {
    match e {
        &Node::Atom(ref s) => parse_literal(&s).map(
            |l| syntax::Quotation::Literal(l)),
        &Node::List(ref s) => parse_quotation_list(&s[..]),
    }
}

fn parse_quotation_list(e: &[Node]) -> Result<syntax::Quotation> {
    match e.split_first() {
        None => Ok(syntax::Quotation::Nil),
        Some((hd, tl)) => Ok(syntax::Quotation::Cons(
            Box::new(try!(parse_quotation(hd))), Box::new(try!(parse_quotation_list(tl))))),
    }
}

fn parse_expression_from_list(hd: &Node, tl: &[Node]) -> Result<syntax::Expression> {
    match hd {
        &Node::Atom(ref keyword) =>
            if keyword == "quote" {
                Ok(syntax::Expression::Quote(try!(parse_quotation_list(tl))))
            } else {
                Err(SchemeError::Basic(format!("unhandled keyword {}", keyword)))
            },
        &Node::List(_) =>
            Err(SchemeError::Basic("Application not implemented".to_string()))
    }
}

pub fn parse_expression(n: Node) -> Result<syntax::Expression> {
    match n {
        Node::Atom(s) =>
            parse_literal_expression(&s)
            .or(parse_variable(s))
            .or(Err(SchemeError::Basic("Invalid atom".to_string()))),
        Node::List(s) =>
            match s.split_first() {
                None => Err(SchemeError::Basic("Unexpected Nil".to_string())),
                Some((hd, tl)) => parse_expression_from_list(hd, tl),
            },
    }
}
