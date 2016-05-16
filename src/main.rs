use std::io::BufRead;
mod lexer;
use lexer::RegexLexer;
mod parser;
use parser::Parser;
// mod io;
// use io::CharsWrap;
mod scheme;
use scheme::parse_expression;

// fn read_stdin() -> String {
//     let mut s = String::new();
//     std::io::stdin().read_to_string(&mut s).unwrap();
//     s
// }

fn main() {
    let stdin = std::io::stdin();
    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();
        let lexer = RegexLexer::new(&line);
        let parser = Parser::new(lexer);
        for node_result in parser {
            match node_result {
                Err(e) => println!("error: {}", e),
                Ok(node) =>
                    match parse_expression(node) {
                        Err(e) => println!("error: {}", e),
                        Ok(expr) => println!("{:?}", expr),
                    },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::{Node, Parser};
    use lexer::RegexLexer;

    fn parse(s: &str) -> Node {
        let lexer = RegexLexer::new(s);
        let mut parser = Parser::new(lexer);
        parser.next().unwrap().unwrap()
    }

    fn expr(s: &str) -> Expression {
        super::scheme::parse_expression(parse(s)).unwrap()
    }

    #[test]
    fn parse_test() {
        parse("(foo bar)");
        parse("foo");
        parse("foo )garbage");
    }

    use super::scheme::syntax::{Expression, Literal};

    #[test]
    fn number() {
        assert_eq!(expr("12"), Expression::Literal(Literal::Number(12)));
    }

    #[test]
    fn bool_true() {
        assert_eq!(expr("#t"), Expression::Literal(Literal::Boolean(true)));
    }

    #[test]
    fn bool_false() {
        assert_eq!(expr("#f"), Expression::Literal(Literal::Boolean(false)));
    }

    #[test]
    fn char_normal() {
        assert_eq!(expr("#\\a"), Expression::Literal(Literal::Character('a')));
    }

    #[test]
    fn char_nl() {
        assert_eq!(expr("#\\newline"), Expression::Literal(Literal::Character('\n')));
    }

    #[test]
    fn char_nl_upper() {
        assert_eq!(expr("#\\NewLine"), Expression::Literal(Literal::Character('\n')));
    }

    #[test]
    fn char_space() {
        assert_eq!(expr("#\\SPace"), Expression::Literal(Literal::Character(' ')));
    }

    #[test]
    fn strings() {
        assert_eq!(expr("\"a\\nb\\tc\""), Expression::Literal(Literal::String("a\nb\tc".to_string())));
    }

    #[test]
    fn var() {
        assert_eq!(expr("foobar"), Expression::Variable("foobar".to_string()));
    }
}
