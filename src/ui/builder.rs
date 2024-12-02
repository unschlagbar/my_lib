use crate::graphics::formats::RGBA;

use super::{style::Absolute, Style};


pub struct UiBuilder {

}

impl UiBuilder {
    pub fn style() -> Style {
        Style::Absolute(Absolute{ x: super::Align::Top(super::UIUnit::Pixel(10.0)), y: super::Align::Right(super::UIUnit::Pixel(10.0)), width: super::UiSize::Size(super::UIUnit::Pixel(150.0)), height: super::UiSize::Size(super::UIUnit::Pixel(350.0)), color: RGBA::new(120, 120, 120, 255), border_color: RGBA::new(180, 180, 180, 255), border: [1.0; 4], corner: [super::UIUnit::Pixel(6.0); 4] })
    }
}