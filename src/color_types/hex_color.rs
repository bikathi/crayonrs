use crate::{
    color_types::rgb_color::Rgb,
    util::{Color, ConversionError, HEX_CHAR_TABLE, IntoColor, index_of_char},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Hex {
    pub value: String,
}

impl Hex {
    pub fn new(hex_string: String) -> Self {
        Self { value: hex_string }
    }

    pub fn hex_pair_to_u8(s: &str) -> Result<u8, ConversionError> {
        let op_result = u8::from_str_radix(s, 16);
        if op_result.is_err() {
            return Err(ConversionError::HexTableIndexingError);
        }

        Ok(op_result.unwrap())
    }
}

fn division_by_16(input: &u8) -> (u8, u8) {
    let quotient = input / 16;
    let remainder = input % 16;
    (quotient, remainder)
}

impl Color for Hex {
    type Input = Self;

    fn pre_check(&self) -> bool {
        if !self.value.starts_with("#") || self.value[1..].len() < 6 {
            return false;
        }

        true
    }

    fn from_rgb(rgb: Rgb) -> Result<Self, crate::util::ConversionError> {
        let mut output = String::from("#");

        for color_space in [rgb.r, rgb.g, rgb.b] {
            let (quotient, remainder) = division_by_16(&color_space);
            // first character of color space
            output.push(HEX_CHAR_TABLE[quotient as usize] as char); // it's an ASCII char always

            // second character of color space
            output.push(HEX_CHAR_TABLE[remainder as usize] as char);
        }

        return Ok(Hex::new(output));
    }

    fn to_rgb(&self) -> Result<Rgb, crate::util::ConversionError> {
        if Hex::pre_check(self) {
            let first_pair = u8::from_str_radix(&self.value[1..=2], 16).unwrap();
            let second_pair = u8::from_str_radix(&self.value[3..=4], 16).unwrap();
            let third_pair = u8::from_str_radix(&self.value[5..=6], 16).unwrap();
            return Ok(Rgb::new(first_pair, second_pair, third_pair));
        }

        Err(ConversionError::InvalidInput)
    }
}

impl IntoColor for Hex {}
