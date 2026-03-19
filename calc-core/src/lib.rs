pub mod ast;
pub mod parser;
pub mod tokenizer;
pub mod evaluator;
pub mod format;

pub use tokenizer::{tokenize, inject_implicit_multiplication, Token};
pub use ast::{Node, BinOp, UnaryOp};
pub use parser::{Parser, ParseError};
pub use evaluator::{Evaluator, EvalError};

#[derive(Debug, Clone)]
pub struct EvalResult {
    pub hex: String,
    pub dec: String,
    pub error: Option<String>,
}

pub fn evaluate(expression: &str, bit_depth: u32, is_degree: bool) -> EvalResult {
    match tokenize(expression) {
        Ok(tokens) => {
            let tokens = inject_implicit_multiplication(tokens);
            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(ast) => {
                    let mut evaluator = Evaluator::new(is_degree);
                    match evaluator.eval(&ast) {
                        Ok(val) => EvalResult {
                            hex: format::to_hex(val, bit_depth),
                            dec: format::to_dec(val, bit_depth),
                            error: None,
                        },
                        Err(e) => EvalResult {
                            hex: String::new(),
                            dec: String::new(),
                            error: Some(format!("{:?}", e)),
                        }
                    }
                }
                Err(e) => EvalResult {
                    hex: String::new(),
                    dec: String::new(),
                    error: Some(format!("{:?}", e)),
                }
            }
        }
        Err(e) => EvalResult {
            hex: String::new(),
            dec: String::new(),
            error: Some(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer() {
        let t = tokenize("2+FF").unwrap();
        assert_eq!(t.len(), 3);
        assert_eq!(t[0], Token::Num(2.0));
        assert_eq!(t[1], Token::Op('+'));
        assert_eq!(t[2], Token::Hex("FF".to_string()));
    }

    #[test]
    fn test_parser() {
        let tokens = tokenize("2+3*4").unwrap();
        let mut p = Parser::new(tokens);
        let ast = p.parse().unwrap();
        match ast {
            Node::BinaryOp(_, BinOp::Add, _) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_evaluator() {
        let res = evaluate("2+3*4", 64, false);
        assert_eq!(res.dec, "14");
    }

    #[test]
    fn test_format() {
        assert_eq!(format::to_hex(255.0, 32), "FF");
        assert_eq!(format::to_dec(255.0, 32), "255");
    }
}
