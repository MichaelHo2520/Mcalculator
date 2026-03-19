#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Num(f64),
    Hex(String), // without 0x
    Op(char),    // '+', '-', '*', '/', '%'
    BitOp(char), // '^', '|', '&'
    Fn(String),  // "SIN", "COS", "TAN", "LOG", "EXP", "SQRT"
    Const(String), // "PI"
    LParen,
    RParen,
    Factorial, // '!'
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() { i += 1; continue; }

        match c {
            '+' | '-' | '*' | '/' | '%' => { tokens.push(Token::Op(c)); i += 1; }
            '^' | '|' | '&' => { tokens.push(Token::BitOp(c)); i += 1; }
            '(' => { tokens.push(Token::LParen); i += 1; }
            ')' => { tokens.push(Token::RParen); i += 1; }
            '!' => { tokens.push(Token::Factorial); i += 1; }
            '0' if i + 1 < chars.len() && (chars[i+1] == 'x' || chars[i+1] == 'X') => {
                i += 2;
                let mut hex_str = String::new();
                while i < chars.len() && chars[i].is_ascii_hexdigit() {
                    hex_str.push(chars[i]);
                    i += 1;
                }
                tokens.push(Token::Hex(hex_str));
            }
            _ if c.is_ascii_alphanumeric() || c == '.' => {
                let mut s = String::new();
                let mut has_dot = false;
                while i < chars.len() && (chars[i].is_ascii_alphanumeric() || chars[i] == '.') {
                    if chars[i] == '.' { has_dot = true; }
                    s.push(chars[i]);
                    i += 1;
                }
                
                let s_upper = s.to_uppercase();
                if s_upper == "PI" {
                    tokens.push(Token::Const(s_upper));
                } else if ["SIN", "COS", "TAN", "LOG", "EXP", "SQRT"].contains(&s_upper.as_str()) {
                    tokens.push(Token::Fn(s_upper));
                } else if !has_dot && s.chars().all(|ch| ch.is_ascii_hexdigit()) && s.chars().any(|ch| ch.is_ascii_alphabetic()) {
                    tokens.push(Token::Hex(s_upper));
                } else {
                    if let Ok(num) = s.parse::<f64>() {
                        tokens.push(Token::Num(num));
                    } else if !has_dot && s.chars().all(|ch| ch.is_ascii_hexdigit()) {
                        tokens.push(Token::Hex(s_upper));
                    } else {
                        return Err(format!("Invalid token: {}", s));
                    }
                }
            }
            _ => return Err(format!("Unknown character: {}", c)),
        }
    }
    Ok(tokens)
}

pub fn inject_implicit_multiplication(tokens: Vec<Token>) -> Vec<Token> {
    let mut res = Vec::new();
    for (i, t) in tokens.iter().enumerate() {
        if i > 0 {
            let prev = &tokens[i-1];
            let needs_mult = match (prev, t) {
                (Token::Num(_), Token::Fn(_)) |
                (Token::Num(_), Token::Const(_)) |
                (Token::Num(_), Token::LParen) |
                (Token::Hex(_), Token::Fn(_)) |
                (Token::Hex(_), Token::Const(_)) |
                (Token::Hex(_), Token::LParen) |
                (Token::RParen, Token::Fn(_)) |
                (Token::RParen, Token::Const(_)) |
                (Token::RParen, Token::LParen) |
                (Token::Factorial, Token::Fn(_)) |
                (Token::Factorial, Token::Const(_)) |
                (Token::Factorial, Token::LParen) |
                (Token::Const(_), Token::Fn(_)) |
                (Token::Const(_), Token::LParen) |
                (Token::Const(_), Token::Num(_)) |
                (Token::Const(_), Token::Hex(_)) => true,
                _ => false,
            };
            if needs_mult {
                res.push(Token::Op('*'));
            }
        }
        res.push(t.clone());
    }
    res
}
