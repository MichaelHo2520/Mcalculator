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
    pub truncated: bool,
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
                                truncated: res.truncated,
                            }
                        }
                        Err(e) => EvalResult {
                            hex: "---".to_string(),
                            dec: "Error".to_string(),
                            error: Some(format!("{:?}", e)),
                            overflowed: false,
                            truncated: false,
                        }
                    }
                }
                Err(e) => EvalResult {
                    hex: "---".to_string(),
                    dec: "Error".to_string(),
                    error: Some(format!("{:?}", e)),
                    overflowed: false,
                    truncated: false,
                }
            }
        }
        Err(e) => EvalResult {
            hex: "---".to_string(),
            dec: "Error".to_string(),
            error: Some(e),
            overflowed: false,
            truncated: false,
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

    // ── 三角函數 (角度模式) ─────────────────────────────────────────

    /// 特殊角度 sin — 浮點誤差應被 round_near_zero 捨入為 0
    #[test]
    fn test_trig_sin_degree_zero_angles() {
        // sin(0°) = 0
        assert_eq!(evaluate("sin(0)", 64, true, true, true).dec, "0");
        // sin(180°) ≈ 1.22e-16 → epsilon 捨入 → 0
        assert_eq!(evaluate("sin(180)", 64, true, true, true).dec, "0");
        // sin(360°) ≈ -2.45e-16 → epsilon 捨入 → 0
        assert_eq!(evaluate("sin(360)", 64, true, true, true).dec, "0");
        // sin(-180°) → 0
        assert_eq!(evaluate("sin(0-180)", 64, true, true, true).dec, "0");
    }

    /// 特殊角度 cos — 浮點誤差應被 round_near_zero 捨入為 0
    #[test]
    fn test_trig_cos_degree_zero_angles() {
        // cos(90°) ≈ 6.12e-17 → epsilon 捨入 → 0
        assert_eq!(evaluate("cos(90)", 64, true, true, true).dec, "0");
        // cos(270°) ≈ -1.84e-16 → epsilon 捨入 → 0
        assert_eq!(evaluate("cos(270)", 64, true, true, true).dec, "0");
    }

    /// 特殊角度 tan — 浮點誤差應被 round_near_zero 捨入為 0
    #[test]
    fn test_trig_tan_degree_zero_angles() {
        // tan(0°) = 0
        assert_eq!(evaluate("tan(0)", 64, true, true, true).dec, "0");
        // tan(180°) = sin(180°)/cos(180°) ≈ 1.22e-16 / -1 → epsilon 捨入 → 0
        assert_eq!(evaluate("tan(180)", 64, true, true, true).dec, "0");
        // tan(360°) → 0
        assert_eq!(evaluate("tan(360)", 64, true, true, true).dec, "0");
    }

    /// 標準角度應有正確的非零結果
    #[test]
    fn test_trig_degree_standard_values() {
        // sin(90°) = 1 (IEEE 754 sin(π/2) 精確為 1.0)
        assert_eq!(evaluate("sin(90)", 64, true, true, true).dec, "1");
        // sin(-90°) = -1
        assert_eq!(evaluate("sin(0-90)", 64, true, true, true).dec, "-1");
        // cos(0°) = 1
        assert_eq!(evaluate("cos(0)", 64, true, true, true).dec, "1");
        // cos(180°) = -1 (IEEE 754 cos(π) 精確為 -1.0)
        assert_eq!(evaluate("cos(180)", 64, true, true, true).dec, "-1");
        // cos(360°) ≈ 0.9999999999999999 (cos(2π) 在 f64 有微小誤差，不為精確 1.0)
        // 驗證結果不為 "0" 且非常接近 1.0
        let cos360 = evaluate("cos(360)", 64, true, true, true);
        let v: f64 = cos360.dec.parse().expect("cos(360) DEC 應可解析");
        assert!((v - 1.0).abs() < 1e-10, "cos(360°) 應接近 1.0，實際: {}", v);
        // tan(45°) ≈ 0.9999999999999999 (f64 tan(π/4) 有微小誤差)
        // 驗證結果非常接近 1.0 且不為 "0"
        let tan45 = evaluate("tan(45)", 64, true, true, true);
        let v: f64 = tan45.dec.parse().expect("tan(45) DEC 應可解析");
        assert!((v - 1.0).abs() < 1e-10, "tan(45°) 應接近 1.0，實際: {}", v);
        assert_ne!(tan45.dec, "0", "tan(45°) 不應被 epsilon 捨入為 0");
    }

    /// sin(30°) = 0.5 — 一般小數不應受 epsilon 影響
    #[test]
    fn test_trig_sin_30_not_zeroed() {
        let res = evaluate("sin(30)", 64, true, true, true);
        // 結果應接近 0.5，且不為 "0"
        assert_ne!(res.dec, "0", "sin(30°) 不應被捨入為 0");
        let v: f64 = res.dec.parse().expect("DEC 應可解析為浮點數");
        assert!((v - 0.5).abs() < 1e-6, "sin(30°) 應接近 0.5，實際: {}", v);
    }

    // NOTE: asin/acos/atan 在 tokenizer 中不被識別為函式 token，
    // 屬於 UI 按鈕觸發的路徑，不可透過文字輸入測試。

    // ── 三角函數 (弧度模式) ─────────────────────────────────────────

    #[test]
    fn test_trig_radian_mode() {
        // 弧度模式下不做角度轉換，sin(0) 仍為 0
        assert_eq!(evaluate("sin(0)", 64, true, false, true).dec, "0");
        // cos(0) = 1
        assert_eq!(evaluate("cos(0)", 64, true, false, true).dec, "1");
        // tan(0) = 0
        assert_eq!(evaluate("tan(0)", 64, true, false, true).dec, "0");
    }

    // ── f32 vs f64 DEC 精度差異 ──────────────────────────────────────

    /// f32 與 f64 的 DEC 顯示應不同，以反映精度損失
    #[test]
    fn test_f32_vs_f64_dec_different() {
        let f32_res = evaluate("1/3", 32, true, false, true);
        let f64_res = evaluate("1/3", 64, true, false, true);
        assert_ne!(
            f32_res.dec, f64_res.dec,
            "f32 與 f64 的 DEC 應不同 (f32={}, f64={})",
            f32_res.dec, f64_res.dec
        );
    }

    /// f64 的 1/3 應有更多有效位數
    #[test]
    fn test_f64_dec_precision() {
        let res = evaluate("1/3", 64, true, false, true);
        // f64 至少有 15 位有效數字，字串長度遠大於 f32
        assert!(
            res.dec.len() > 10,
            "f64 DEC 應有更多位數，實際: {}",
            res.dec
        );
        assert!(res.dec.starts_with("0.333333333"), "f64 DEC 應以 0.333333333 開頭，實際: {}", res.dec);
    }

    /// f32 的 1/3 顯示與 f64 不同，反映精度差異（第 7 位起數字不同）
    #[test]
    fn test_f32_dec_precision() {
        let f32_res = evaluate("1/3", 32, true, false, true);
        let f64_res = evaluate("1/3", 64, true, false, true);
        // f32 DEC 應為 f32 精度轉換後的值，不應等同 f64
        assert_ne!(f32_res.dec, f64_res.dec, "f32 與 f64 DEC 不應相同");
        // f64 應有更多連續的 3：0.3333333333333333
        assert!(
            f64_res.dec.starts_with("0.333333333"),
            "f64 DEC 應有更多 3，實際: {}",
            f64_res.dec
        );
        // f32 的第 8 位數字應與 f64 不同（精度損失造成偏差）
        let f32_digits: Vec<char> = f32_res.dec.chars().collect();
        let f64_digits: Vec<char> = f64_res.dec.chars().collect();
        assert_ne!(
            f32_digits.get(9), f64_digits.get(9),
            "f32 與 f64 在第 8 位有效數字後應出現差異 (f32={}, f64={})",
            f32_res.dec, f64_res.dec
        );
    }

    /// HEX 位數：f32 = 8碼，f64 = 16碼
    #[test]
    fn test_float_hex_width() {
        let f32_res = evaluate("1/3", 32, true, false, true);
        let f64_res = evaluate("1/3", 64, true, false, true);
        assert_eq!(f32_res.hex.len(), 8, "f32 HEX 應為 8碼，實際: {}", f32_res.hex);
        assert_eq!(f64_res.hex.len(), 16, "f64 HEX 應為 16碼，實際: {}", f64_res.hex);
    }

    // ── 浮點模式下 hex 輸入永遠當整數值 ──────────────────────────────

    /// 0x3C 在 f32/f64 模式下應視為整數 60，而非 IEEE 754 位元模式
    #[test]
    fn test_float_hex_as_integer() {
        // f32 模式: 1/0x3C = 1/60
        let f32_res = evaluate("1/0x3C", 32, true, false, true);
        assert!(f32_res.error.is_none(), "f32 1/0x3C 不應出錯: {:?}", f32_res.error);
        let f32_dec: f64 = f32_res.dec.parse().expect("f32 DEC 應為有效數字");
        assert!((f32_dec - 1.0/60.0).abs() < 1e-6, "f32 1/0x3C 應≈0.01667，實際: {}", f32_res.dec);

        // f64 模式: 1/0x3C = 1/60
        let f64_res = evaluate("1/0x3C", 64, true, false, true);
        assert!(f64_res.error.is_none(), "f64 1/0x3C 不應出錯: {:?}", f64_res.error);
        let f64_dec: f64 = f64_res.dec.parse().expect("f64 DEC 應為有效數字");
        assert!((f64_dec - 1.0/60.0).abs() < 1e-10, "f64 1/0x3C 應≈0.01667，實際: {}", f64_res.dec);

        // f32 與 f64 的精度應有差異（DEC 或 HEX 不同）
        assert_ne!(f32_res.hex, f64_res.hex, "f32 與 f64 的 HEX 輸出應不同");
    }

    /// 即使 hex 位數剛好等於 IEEE 754 寬度，仍當整數解讀
    #[test]
    fn test_float_hex_full_width_still_integer() {
        // 0x3F800000 (8位) 在 f32 模式 = 整數 1065353216，非 IEEE 754 的 1.0
        let res = evaluate("0x3F800000", 32, true, false, true);
        let dec: f64 = res.dec.parse().expect("應為有效數字");
        assert!((dec - 1065353216.0).abs() < 1.0, "f32 0x3F800000 應為整數 1065353216，實際: {}", res.dec);
    }

    // ── 超小數 DEC 顯示格式 ──────────────────────────────────────────

    /// 不含 epsilon 捨入範圍但極小的浮點數應以科學記號顯示
    #[test]
    fn test_small_float_scientific_notation() {
        // 1e-8 < 1e-6，應顯示為科學記號
        let res = evaluate("1/100000000", 64, true, false, true);
        assert!(
            res.dec.contains('e') || res.dec.contains('E'),
            "超小數應以科學記號顯示，實際: {}",
            res.dec
        );
        // 0.001 >= 1e-6，應以一般小數顯示
        let res2 = evaluate("1/1000", 64, true, false, true);
        assert_eq!(res2.dec, "0.001", "0.001 應以一般格式顯示，實際: {}", res2.dec);
    }
}
