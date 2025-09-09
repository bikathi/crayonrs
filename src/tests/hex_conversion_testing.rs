use crate::{
    color_types::{hex_color::Hex, rgb_color::Rgb},
    util::IntoColor,
};

#[test]
fn should_get_hex_from_rgb() {
    let white = Rgb::new(255, 255, 255);
    let color = white.into_color::<Hex>().unwrap();
    assert_eq!(color.value, "#FFFFFF");
}

#[test]
fn should_get_rgb_from_hex() {
    let hex = Hex::new(String::from("#FFFFFF"));
    let rgb = hex.into_color::<Rgb>().unwrap();
    assert_eq!(rgb, Rgb::new(255, 255, 255));
}
