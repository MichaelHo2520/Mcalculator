#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
    Add, Sub, Mul, Div, Mod,
    BitAnd, BitOr, BitXor,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Pos, Neg,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Num(f64),
    BinaryOp(Box<Node>, BinOp, Box<Node>),
    UnaryOp(UnaryOp, Box<Node>),
    FnCall(String, Box<Node>),
    Factorial(Box<Node>),
}
