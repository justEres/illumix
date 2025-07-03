#[derive(Debug)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn hex_to_rgb(hex_code: &str) -> RGB {
    let hex_r = &hex_code[1..3];
    let hex_g = &hex_code[3..5];
    let hex_b = &hex_code[5..7];

    let rgb = RGB {
        r: u8::from_str_radix(hex_r, 16).unwrap(),
        g: u8::from_str_radix(hex_g, 16).unwrap(),
        b: u8::from_str_radix(hex_b, 16).unwrap(),
    };

    return rgb;
}

pub fn rgb_to_hex(r: u8, g: u8, b: u8) -> String{

    let hex_code: String = format!("#{:02x}{:02x}{:02x}", r, g, b);

    return hex_code;
}
