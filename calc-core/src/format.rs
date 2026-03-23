pub struct FormatResult {
    pub hex: String,
    pub dec: String,
    pub overflowed: bool,
    pub truncated: bool,
}

/// 以 N 位有效數字格式化浮點數，移除尾端多餘的零
fn format_significant(val: f64, digits: usize) -> String {
    if val == 0.0 {
        return "0.0".to_string();
    }
    // 科學記號門檻
    if val.abs() >= 1e15 || val.abs() < 1e-6 {
        // 有效位數 = digits-1 (科學記號小數部分)
        let s = format!("{:.prec$e}", val, prec = digits.saturating_sub(1));
        return s;
    }
    // 計算整數部分佔多少位
    let int_digits = if val.abs() >= 1.0 {
        (val.abs().log10().floor() as usize) + 1
    } else {
        0
    };
    let decimal_places = if digits > int_digits { digits - int_digits } else { 0 };
    let s = format!("{:.prec$}", val, prec = decimal_places);
    // 移除尾端零 (保留至少一位小數點後的 0 以表示浮點)
    if s.contains('.') {
        let trimmed = s.trim_end_matches('0');
        if trimmed.ends_with('.') {
            trimmed.to_string() + "0"
        } else {
            trimmed.to_string()
        }
    } else {
        s
    }
}

pub fn get_mask(bit_depth: u32) -> u64 {
    if bit_depth >= 64 {
        u64::MAX
    } else {
        (1u64 << bit_depth) - 1
    }
}

pub fn truncate_and_format(val: f64, bit_depth: u32, is_signed: bool, is_float: bool) -> FormatResult {
    // 1. 特殊值檢查
    if val.is_nan() || !val.is_finite() {
        if is_float {
            return FormatResult {
                hex: if bit_depth == 32 { format!("{:08X}", (val as f32).to_bits()) } else { format!("{:016X}", val.to_bits()) },
                dec: if val.is_nan() { "NaN".to_string() } else if val.is_sign_positive() { "Infinity".to_string() } else { "-Infinity".to_string() },
                overflowed: !val.is_nan(),
                truncated: false,
            };
        } else {
            return FormatResult {
                hex: "Error".to_string(),
                dec: "Error".to_string(),
                overflowed: false,
                truncated: false,
            };
        }
    }

    let truncated = !is_float && val != val.trunc();
    let mut trunc_val = if is_float { val } else { val.trunc() };

    // Fix negative zero display
    if trunc_val == 0.0 {
        trunc_val = 0.0;
    }

    // 2. 決定上下界 (f64)
    let min_val: f64;
    let max_val: f64;
    if is_float {
        min_val = f64::NEG_INFINITY;
        max_val = f64::INFINITY;
    } else if is_signed {
        if bit_depth >= 64 {
            min_val = -9223372036854775808.0; // i64::MIN
            max_val = 9223372036854775808.0;  // i64::MAX as f64 (2^63)
        } else {
            min_val = -(1i64 << (bit_depth - 1)) as f64;
            max_val = ((1i64 << (bit_depth - 1)) - 1) as f64;
        }
    } else {
        if bit_depth >= 64 {
            min_val = 0.0;
            max_val = 18446744073709551616.0; // u64::MAX as f64 (2^64)
        } else {
            min_val = 0.0;
            max_val = ((1u64 << bit_depth) - 1) as f64;
        }
    }

    // 3. 溢位偵測
    let overflowed = if is_float {
        false
    } else if is_signed && bit_depth >= 64 {
        trunc_val < min_val || trunc_val >= max_val
    } else if !is_signed && bit_depth >= 64 {
        trunc_val < min_val || trunc_val >= max_val
    } else {
        trunc_val < min_val || trunc_val > max_val
    };

    // 4. DEC 格式化
    //    f32: 顯示真實儲存值 (f32→f64)，限 9 位有效數字以揭示精度損失
    //    f64: 顯示完整精度，限 17 位有效數字
    //    整數: 直接輸出截斷後的整數
    let dec = if is_float && bit_depth == 32 {
        let real_val = (trunc_val as f32) as f64;
        format_significant(real_val, 9)
    } else if is_float {
        format_significant(trunc_val, 17)
    } else {
        format!("{}", trunc_val as i128)
    };

    // 5. HEX 格式化
    let hex = if overflowed {
        "溢位".to_string()
    } else {
        if is_float {
            if bit_depth == 32 {
                format!("{:08X}", (trunc_val as f32).to_bits())
            } else {
                format!("{:016X}", trunc_val.to_bits())
            }
        } else {
            let hex_width = (bit_depth / 4) as usize;
            let mask = get_mask(bit_depth);
            
            let raw_hex = if is_signed {
                (trunc_val as i64) as u64
            } else {
                trunc_val as u64
            };
            
            let truncated_hex = raw_hex & mask;
            format!("{:0>width$X}", truncated_hex, width = hex_width)
        }
    };

    FormatResult {
        hex,
        dec,
        overflowed,
        truncated,
    }
}
