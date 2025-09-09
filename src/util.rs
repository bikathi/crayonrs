use crate::color_types::rgb_color::Rgb;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConversionError {
    #[error("input may be invalid!")]
    InvalidInput,

    #[error("failed to convert from hex")]
    HexTableIndexingError,
}

// we are using the hub-and-spoke pattern to handle color conversions
// for example, to convert from hs(var) to hex, we do hs(var) -> rgb -> hex
// therefore, all supported color types must implement this trait
pub trait Color: Clone + PartialEq {
    type Input;

    /// Before conversion, check that `input` is valid
    fn pre_check(&self) -> bool {
        true
    }

    /// Convert this color to RGB as an intermediate format
    fn to_rgb(&self) -> Result<Rgb, ConversionError>;

    /// Create this color from RGB
    fn from_rgb(rgb: Rgb) -> Result<Self, ConversionError>;
}

pub trait IntoColor: Color {
    fn into_color<T: Color>(self) -> Result<T, ConversionError> {
        into_color(self)
    }
}

// convert between one color type to the other
fn into_color<From: Color, To: Color>(from: From) -> Result<To, ConversionError> {
    To::from_rgb(from.to_rgb().unwrap())
}

// the list of hexadecimal characters
pub const HEX_CHAR_TABLE: &[u8; 16] = b"0123456789ABCDEF";
pub const MAX_FOR_RGB_COLOR_SPACE: u8 = 255u8;

pub fn index_of_char(character: char) -> Result<usize, ConversionError> {
    if character == '#' {
        return Err(ConversionError::HexTableIndexingError);
    }

    // Linear search -> as array isn't that large
    for (index, &table_char) in HEX_CHAR_TABLE.iter().enumerate() {
        if table_char == character as u8 {
            return Ok(index);
        }
    }

    Err(ConversionError::HexTableIndexingError)
}
