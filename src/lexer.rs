use std;
extern crate regex;
use self::regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(i32),
    Boolean(bool),
    Character(char),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Identifier(String),
    LParen,
    RParen,
    Literal(Literal),
}

pub type Result<T> = std::result::Result<T, &'static str>;

pub trait Lexer : Iterator<Item=Result<Token>> {}
impl <I> Lexer for I where I: Iterator<Item=Result<Token>> {}

pub struct RegexLexer<'t> {
    lexer_re: Regex,
    rest: &'t str,
}

impl <'t> RegexLexer<'t> {
    pub fn new(text: &'t str) -> Self {
        let lexer_re = Regex::new(
            r#"\s*(?xi)
               (?:
               (?P<lparen>\()|
               (?P<rparen>\))|
               (?P<identifier>[a-z!$%&*/:<=>?~_^]
                    [a-z!$%&*/:<=>?~_^0-9.+-]*)|
               (?P<boolean>\#[tf])|
               (?P<number>[0-9]+)|
               (?P<character>\#\\(?:newline|space|.))|
               (?P<string>"(?:[^\\]|\\.)*"))"#).unwrap();

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
            println!("begin != 0: {} {:?} {:?}", begin, self.rest, mo.at(0));
            self.rest = "";
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
            else if groupname == "identifier" {
                Token::Identifier(value.to_string()) }
            else { Token::Literal(parse_literal(groupname, value)) }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer_test() {
        let l = RegexLexer::new("(foo bar)");
        let tokens = l.collect::<Vec<_>>();
        println!("{:?}", tokens);
        assert_eq!(tokens[0], Ok(Token::LParen));
        assert_eq!(tokens[3], Ok(Token::RParen));
    }
}
