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
    pub overflowed: bool,
}

pub fn evaluate(expression: &str, bit_depth: u32, is_signed: bool, is_degree: bool, is_float: bool) -> EvalResult {
    match tokenize(expression) {
        Ok(tokens) => {
            let tokens = inject_implicit_multiplication(tokens);
            let mut parser = Parser::new(tokens, bit_depth, is_signed, is_float);
            match parser.parse() {
                Ok(ast) => {
                    let mut evaluator = Evaluator::new(is_degree);
                    match evaluator.eval(&ast) {
                        Ok(val) => {
                            let res = format::truncate_and_format(val, bit_depth, is_signed, is_float);
                            EvalResult {
                                hex: res.hex,
                                dec: res.dec,
                                error: None,
                                overflowed: res.overflowed,
                            }
                        }
                        Err(e) => EvalResult {
                            hex: "---".to_string(),
                            dec: "Error".to_string(),
                            error: Some(format!("{:?}", e)),
                            overflowed: false,
                        }
                    }
                }
                Err(e) => EvalResult {
                    hex: "---".to_string(),
                    dec: "Error".to_string(),
                    error: Some(format!("{:?}", e)),
                    overflowed: false,
                }
            }
        }
        Err(e) => EvalResult {
            hex: "---".to_string(),
            dec: "Error".to_string(),
            error: Some(e),
            overflowed: false,
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
    fn test_tokenizer_shift() {
        let t = tokenize("1<<8").unwrap();
        assert_eq!(t[1], Token::ShiftOp("<<".to_string()));
    }

    #[test]
    fn test_parser() {
        let tokens = tokenize("2+3*4").unwrap();
        let mut p = Parser::new(tokens, 64, true, false);
        let ast = p.parse().unwrap();
        match ast {
            Node::BinaryOp(_, BinOp::Add, _) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_evaluator() {
        let res = evaluate("2+3*4", 64, true, false, false);
        assert_eq!(res.dec, "14");
    }

    #[test]
    fn test_type_truncation() {
        // uint8 strict overflow 0-1
        assert_eq!(evaluate("0-1", 8, false, false, false).dec, "-1");
        assert_eq!(evaluate("0-1", 8, false, false, false).hex, "溢位");
        // int8 2's complement
        assert_eq!(evaluate("0-1", 8, true, false, false).dec, "-1");
        assert_eq!(evaluate("0-1", 8, true, false, false).hex, "FF");
    }

    #[test]
    fn test_overflow_detection() {
        assert_eq!(evaluate("0-1", 8, false, false, false).overflowed, true);
        assert_eq!(evaluate("0-1", 8, true, false, false).overflowed, false);
        assert_eq!(evaluate("128", 8, true, false, false).overflowed, true);
        assert_eq!(evaluate("256", 8, false, false, false).overflowed, true);
        assert_eq!(evaluate("-128", 8, true, false, false).overflowed, false);
        assert_eq!(evaluate("-129", 8, true, false, false).overflowed, true);
    }

    #[test]
    fn test_hex_padding() {
        assert_eq!(evaluate("10", 8, true, false, false).hex, "0A");
        assert_eq!(evaluate("10", 16, true, false, false).hex, "000A");
        assert_eq!(evaluate("10", 32, true, false, false).hex, "0000000A");
        assert_eq!(evaluate("10", 64, true, false, false).hex, "000000000000000A");
    }

    #[test]
    fn test_integer_division() {
        assert_eq!(evaluate("7/2", 32, true, false, false).dec, "3");
    }

    #[test]
    fn test_bit_ops() {
        // 1<<8 in u16: 256
        assert_eq!(evaluate("1<<8", 16, false, false, false).dec, "256");
        assert_eq!(evaluate("256>>4", 16, false, false, false).dec, "16");
        // ~0 in u8: is -1, overflows u8 unsigned, returns -1, "溢位"
        assert_eq!(evaluate("~0", 8, false, false, false).dec, "-1");
        assert_eq!(evaluate("~0", 8, false, false, false).hex, "溢位");
        // ~0 in i8: is -1, no overflow, returns "-1", "FF"
        assert_eq!(evaluate("~0", 8, true, false, false).dec, "-1");
        assert_eq!(evaluate("~0", 8, true, false, false).hex, "FF");
    }
}
