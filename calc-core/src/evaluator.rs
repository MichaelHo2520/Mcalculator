use crate::ast::{Node, BinOp, UnaryOp};

#[derive(Debug)]
pub enum EvalError {
    DivisionByZero,
    InvalidExpression,
}

pub struct Evaluator {
    is_degree: bool,
}

impl Evaluator {
    pub fn new(is_degree: bool) -> Self {
        Evaluator { is_degree }
    }

    pub fn eval(&mut self, node: &Node) -> Result<f64, EvalError> {
        match node {
            Node::Num(n) => Ok(*n),
            Node::BinaryOp(left, op, right) => {
                let l = self.eval(left)?;
                let r = self.eval(right)?;
                match op {
                    BinOp::Add => Ok(l + r),
                    BinOp::Sub => Ok(l - r),
                    BinOp::Mul => Ok(l * r),
                    BinOp::Div => if r == 0.0 { Err(EvalError::DivisionByZero) } else { Ok(l / r) },
                    BinOp::Mod => if r == 0.0 { Err(EvalError::DivisionByZero) } else { Ok(l % r) },
                    BinOp::BitAnd => Ok((l.trunc() as i64 & r.trunc() as i64) as f64),
                    BinOp::BitOr => Ok((l.trunc() as i64 | r.trunc() as i64) as f64),
                    BinOp::BitXor => Ok((l.trunc() as i64 ^ r.trunc() as i64) as f64),
                }
            }
            Node::UnaryOp(UnaryOp::Pos, expr) => self.eval(expr),
            Node::UnaryOp(UnaryOp::Neg, expr) => Ok(-self.eval(expr)?),
            Node::FnCall(name, expr) => {
                let mut v = self.eval(expr)?;
                if self.is_degree && (name == "SIN" || name == "COS" || name == "TAN") {
                    v = v.to_radians();
                }
                match name.as_str() {
                    "SIN" => Ok(v.sin()),
                    "COS" => Ok(v.cos()),
                    "TAN" => Ok(v.tan()),
                    "LOG" => Ok(v.log10()),
                    "EXP" => Ok(v.exp()),
                    "SQRT" => Ok(v.sqrt()),
                    "ASIN" => Ok(if self.is_degree { v.asin().to_degrees() } else { v.asin() }),
                    "ACOS" => Ok(if self.is_degree { v.acos().to_degrees() } else { v.acos() }),
                    "ATAN" => Ok(if self.is_degree { v.atan().to_degrees() } else { v.atan() }),
                    _ => Err(EvalError::InvalidExpression),
                }
            }
            Node::Factorial(expr) => {
                let v = self.eval(expr)?;
                if v < 0.0 { return Ok(f64::NAN); }
                if v.fract() != 0.0 { return Ok(f64::NAN); }
                let mut res = 1.0;
                for i in 2..=(v as i64) { res *= i as f64; }
                Ok(res)
            }
        }
    }
}
