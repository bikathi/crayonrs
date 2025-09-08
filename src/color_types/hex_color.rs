use crate::{
    color_types::rgb_color::Rgb,
    util::{Color, ConversionError, HEX_CHAR_TABLE, IntoColor},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Hex {
    pub value: String,
}

impl Hex {
    fn new(hex_string: String) -> Self {
        Self { value: hex_string }
    }
}

fn division_by_16(input: &u8) -> (u8, u8) {
    let quotient = input / 16;
    let remainder = input % 16;
    (quotient, remainder)
}

impl Color for Hex {
    type Input = Self;

    fn pre_check(input: &Self::Input) -> bool {
        if !input.value.starts_with("#") || &input.value[1..].len() < &6 {
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
        todo!()
    }
}

impl IntoColor for Hex {}
