pub fn get_mask(bit_depth: u32) -> u64 {
    if bit_depth >= 64 { u64::MAX }
    else { (1u64 << bit_depth) - 1 }
}

pub fn to_hex(val: f64, bit_depth: u32) -> String {
    if val.is_nan() || !val.is_finite() { return "Error".to_string(); }
    let v_i64 = val.trunc() as i64;
    let masked = v_i64 as u64 & get_mask(bit_depth);
    format!("{:X}", masked)
}

pub fn to_dec(val: f64, bit_depth: u32) -> String {
    if val.is_nan() || !val.is_finite() { return "Error".to_string(); }
    if val.fract() == 0.0 {
        let v_i64 = val.trunc() as i64;
        let masked = v_i64 as u64 & get_mask(bit_depth);
        let mut sign_val = masked as i64;
        if bit_depth < 64 {
            let sign_bit = 1u64 << (bit_depth - 1);
            if masked & sign_bit != 0 {
                sign_val = (masked | !get_mask(bit_depth)) as i64;
            }
        }
        format!("{}", sign_val)
    } else {
        format!("{}", val)
    }
}
