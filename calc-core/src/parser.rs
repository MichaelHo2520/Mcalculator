use crate::tokenizer::Token;
use crate::ast::{Node, BinOp, UnaryOp};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    UnexpectedEOF,
    MissingRParen,
    InvalidExpression,
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    bit_depth: u32,
    is_signed: bool,
    is_float: bool,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, bit_depth: u32, is_signed: bool, is_float: bool) -> Self {
        Parser { tokens, pos: 0, bit_depth, is_signed, is_float }
    }
    pub fn parse(&mut self) -> Result<Node, ParseError> {
        if self.tokens.is_empty() { return Ok(Node::Num(0.0)); }
        let node = self.parse_expr()?;
        if self.pos < self.tokens.len() {
            Err(ParseError::UnexpectedToken(self.tokens[self.pos].clone()))
        } else {
            Ok(node)
        }
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    fn advance(&mut self) -> Option<&Token> {
        let res = self.tokens.get(self.pos);
        if res.is_some() { self.pos += 1; }
        res
    }
    fn parse_expr(&mut self) -> Result<Node, ParseError> {
        self.parse_bit_or()
    }
    fn parse_bit_or(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_bit_xor()?;
        while let Some(Token::BitOp('|')) = self.peek() {
            self.advance();
            node = Node::BinaryOp(Box::new(node), BinOp::BitOr, Box::new(self.parse_bit_xor()?));
        }
        Ok(node)
    }
    fn parse_bit_xor(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_bit_and()?;
        while let Some(Token::BitOp('^')) = self.peek() {
            self.advance();
            node = Node::BinaryOp(Box::new(node), BinOp::BitXor, Box::new(self.parse_bit_and()?));
        }
        Ok(node)
    }
    fn parse_bit_and(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_shift()?;
        while let Some(Token::BitOp('&')) = self.peek() {
            self.advance();
            node = Node::BinaryOp(Box::new(node), BinOp::BitAnd, Box::new(self.parse_shift()?));
        }
        Ok(node)
    }
    fn parse_shift(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_add()?;
        while let Some(Token::ShiftOp(op)) = self.peek() {
            let op = op.clone();
            self.advance();
            let bin_op = if op == "<<" { BinOp::Shl } else { BinOp::Shr };
            node = Node::BinaryOp(Box::new(node), bin_op, Box::new(self.parse_add()?));
        }
        Ok(node)
    }
    fn parse_add(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_mul()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Op('+') => { self.advance(); node = Node::BinaryOp(Box::new(node), BinOp::Add, Box::new(self.parse_mul()?)); }
                Token::Op('-') => { self.advance(); node = Node::BinaryOp(Box::new(node), BinOp::Sub, Box::new(self.parse_mul()?)); }
                _ => break,
            }
        }
        Ok(node)
    }
    fn parse_mul(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_unary()?;
        while let Some(tok) = self.peek() {
            match tok {
                Token::Op('*') => { self.advance(); node = Node::BinaryOp(Box::new(node), BinOp::Mul, Box::new(self.parse_unary()?)); }
                Token::Op('/') => { self.advance(); node = Node::BinaryOp(Box::new(node), BinOp::Div, Box::new(self.parse_unary()?)); }
                Token::Op('%') => { self.advance(); node = Node::BinaryOp(Box::new(node), BinOp::Mod, Box::new(self.parse_unary()?)); }
                _ => break,
            }
        }
        Ok(node)
    }
    fn parse_unary(&mut self) -> Result<Node, ParseError> {
        if let Some(tok) = self.peek() {
            match tok {
                Token::Op('+') => {
                    self.advance();
                    return Ok(Node::UnaryOp(UnaryOp::Pos, Box::new(self.parse_unary()?)));
                }
                Token::Op('-') => {
                    self.advance();
                    return Ok(Node::UnaryOp(UnaryOp::Neg, Box::new(self.parse_unary()?)));
                }
                Token::BitNot => {
                    self.advance();
                    return Ok(Node::UnaryOp(UnaryOp::BitNot, Box::new(self.parse_unary()?)));
                }
                _ => {}
            }
        }
        self.parse_factorial()
    }
    fn parse_factorial(&mut self) -> Result<Node, ParseError> {
        let mut node = self.parse_primary()?;
        while let Some(Token::Factorial) = self.peek() {
            self.advance();
            node = Node::Factorial(Box::new(node));
        }
        Ok(node)
    }
    fn parse_primary(&mut self) -> Result<Node, ParseError> {
        let tok = self.advance().cloned().ok_or(ParseError::UnexpectedEOF)?;
        match tok {
            Token::Num(n) => Ok(Node::Num(n)),
            Token::Hex(s) => {
                let uval = u64::from_str_radix(&s, 16).map_err(|_| ParseError::InvalidExpression)?;
                // Hex input is always treated as an integer value.
                // IEEE 754 bit interpretation only happens at output (format.rs).
                let fval = if self.is_float {
                    uval as f64
                } else if self.is_signed {
                    let shift = 64 - self.bit_depth;
                    let sval = (uval << shift) as i64 >> shift;
                    sval as f64
                } else {
                    uval as f64
                };
                Ok(Node::Num(fval))
            }
            Token::Const(s) if s == "PI" => Ok(Node::Num(core::f64::consts::PI)),
            Token::Fn(name) => {
                if let Some(Token::LParen) = self.advance().cloned() {
                    match self.parse_expr() {
                        Ok(arg) => {
                            if let Some(Token::RParen) = self.peek() {
                                self.advance(); // consume ')'
                            }
                            Ok(Node::FnCall(name, Box::new(arg)))
                        }
                        Err(ParseError::UnexpectedEOF) => Ok(Node::FnCall(name, Box::new(Node::Num(0.0)))),
                        Err(e) => Err(e),
                    }
                } else {
                    Ok(Node::FnCall(name, Box::new(Node::Num(0.0))))
                }
            }
            Token::LParen => {
                match self.parse_expr() {
                    Ok(node) => {
                        if let Some(Token::RParen) = self.peek() {
                            self.advance();
                        }
                        Ok(node)
                    }
                    Err(ParseError::UnexpectedEOF) => Ok(Node::Num(0.0)),
                    Err(e) => Err(e),
                }
            }
            _ => Err(ParseError::UnexpectedToken(tok))
        }
    }
}
