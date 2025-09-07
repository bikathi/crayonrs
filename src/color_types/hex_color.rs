use crate::{
    color_types::rgb_color::Rgb,
    util::{Color, Convert},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hex<'a> {
    pub value: &'a str,
}

impl<'a> Hex<'a> {}

impl<'a> Color for Hex<'a> {
    fn from_rgb(rgb: Rgb) -> Result<Self, crate::util::ConversionError> {
        todo!()
    }

    fn to_rgb(&self) -> Result<Rgb, crate::util::ConversionError> {
        todo!()
    }
}

impl<'a> Convert for Hex<'a> {}
