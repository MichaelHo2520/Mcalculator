pub struct FormatResult {
    pub hex: String,
    pub dec: String,
    pub overflowed: bool,
}

pub fn get_mask(bit_depth: u32) -> u64 {
    if bit_depth >= 64 {
        u64::MAX
    } else {
        (1u64 << bit_depth) - 1
    }
}

pub fn truncate_and_format(val: f64, bit_depth: u32, is_signed: bool) -> FormatResult {
    // 1. 特殊值檢查
    if val.is_nan() || !val.is_finite() {
        return FormatResult {
            hex: "Error".to_string(),
            dec: "Error".to_string(),
            overflowed: false,
        };
    }

    // 2. 整數截斷
    let raw = val.trunc() as i64;

    // 3. 位元遮罩截斷
    let mask = get_mask(bit_depth);
    let truncated = (raw as u64) & mask;

    // 4. 溢位偵測（值域範圍比對法）
    let overflowed = if is_signed {
        if bit_depth < 64 {
            let min = -(1i64 << (bit_depth - 1));
            let max = (1i64 << (bit_depth - 1)) - 1;
            raw < min || raw > max
        } else {
            // bit_depth == 64, i64 不會溢出 i64
            false
        }
    } else {
        if bit_depth < 64 {
            raw < 0 || (raw as u64) > mask
        } else {
            // bit_depth == 64, 無法直接比 > mask (u64::MAX)
            // 負數在 unsigned 語義下算是溢出
            raw < 0
        }
    };

    // 5. HEX 格式化（嚴格補零）
    let hex_width = (bit_depth / 4) as usize;
    let hex = format!("{:0>width$X}", truncated, width = hex_width);

    // 6. DEC 格式化
    let dec = if is_signed {
        if bit_depth < 64 {
            let sign_bit = 1u64 << (bit_depth - 1);
            if truncated & sign_bit != 0 {
                let signed_val = (truncated | !mask) as i64;
                format!("{}", signed_val)
            } else {
                format!("{}", truncated)
            }
        } else {
            format!("{}", truncated as i64)
        }
    } else {
        format!("{}", truncated)
    };

    FormatResult {
        hex,
        dec,
        overflowed,
    }
}
