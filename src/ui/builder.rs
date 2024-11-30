use crate::graphics::formats::RGB;

use super::Style;


pub struct UiBuilder {

}

impl UiBuilder {
    pub fn style() -> Style {
        Style { x: super::Align::Top(super::UIUnit::Pixel(10.0)), y: super::Align::Right(super::UIUnit::Pixel(10.0)), width: super::UiSize::Size(super::UIUnit::Pixel(150.0)), height: super::UiSize::Size(super::UIUnit::Pixel(350.0)), color: RGB::new(120, 120, 120), border_color: RGB::new(180, 180, 180), border: 1.0, corner: super::UIUnit::Pixel(6.0) }
    }
}