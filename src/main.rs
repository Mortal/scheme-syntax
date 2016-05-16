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

    fn parse(s: &str) -> Result<Node, &'static str> {
        let lexer = RegexLexer::new(s);
        let mut parser = Parser::new(lexer);
        parser.next().unwrap_or(Err("Parser returned None"))
    }

    #[test]
    fn parse_test() {
        println!("{:?}", parse("(foo bar)"));
        parse("(foo bar)").unwrap();
        parse("foo").unwrap();
        parse("foo )garbage").unwrap();
        parse(")").unwrap_err();
        parse("").unwrap_err();
    }
}
