use std::io::Error;

use crate::color_types::rgb_color::Rgb;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("input may be invalid!")]
    InvalidInput,
}

// we are using the hub-and-spoke pattern to handle color conversions
// for example, to convert from hs(var) to hex, we do hs(var) -> rgb -> hex
// therefore, all supported color types must implement this trait
pub trait Color: Clone + Copy + PartialEq {
    /// Convert this color to RGB as an intermediate format
    fn to_rgb(&self) -> Result<Rgb, ConversionError>;

    /// Create this color from RGB
    fn from_rgb(rgb: Rgb) -> Result<Self, ConversionError>;
}

pub trait Convert: Color {
    fn into_color<T: Color>(self) -> Result<T, ConversionError> {
        into_color(self)
    }
}

// convert between one color type to the other
fn into_color<From: Color, To: Color>(from: From) -> Result<To, ConversionError> {
    To::from_rgb(from.to_rgb().unwrap())
}
