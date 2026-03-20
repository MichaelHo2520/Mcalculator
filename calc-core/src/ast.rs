#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Pos, Neg, BitNot,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Num(f64),
    BinaryOp(Box<Node>, BinOp, Box<Node>),
    UnaryOp(UnaryOp, Box<Node>),
    FnCall(String, Box<Node>),
    Factorial(Box<Node>),
}
