use crate::util::{Color, ConversionError, Convert};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red,
            g: green,
            b: blue,
        }
    }
}

impl Color for Rgb {
    fn from_rgb(rgb: Rgb) -> Result<Self, ConversionError> {
        Ok(rgb)
    }

    fn to_rgb(&self) -> Result<Rgb, ConversionError> {
        Ok(*self)
    }
}

impl Convert for Rgb {}
