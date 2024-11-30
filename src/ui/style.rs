use crate::graphics::formats::RGB;

use super::{Align, UIUnit, UiSize};


#[derive(Debug, Clone, Copy)]
pub struct Style {
    pub x: Align,
    pub y: Align,
    pub width: UiSize,
    pub height: UiSize,
    pub color: RGB,
    pub border_color: RGB,
    pub border: f32,
    pub corner: UIUnit,
}

impl Style {
    pub const FULL: Self = Self { x: Align::Left(UIUnit::Zero()), y: Align::Top(UIUnit::Zero()), width: UiSize::Size(UIUnit::Relative(1.0)), height: UiSize::Size(UIUnit::Relative(1.0)), color: RGB::BLACK, border_color: RGB::BLACK, border: 0.0, corner: UIUnit::Zero() };
    
    pub fn infill(padding: f32, color: RGB) -> Self {
        Self { x: Align::Left(UIUnit::Pixel(padding)), y: Align::Top(UIUnit::Pixel(padding)), width: UiSize::Sub(UIUnit::Relative(1.0), UIUnit::Pixel(padding * 2.0)), height: UiSize::Sub(UIUnit::Relative(1.0), UIUnit::Pixel(padding * 2.0)), color, border_color: color, border: 0.0, corner: UIUnit::RelativeHeight(0.5) }
    }

    pub fn toggle(color: RGB) -> Self {
        Self { x: Align::Left(UIUnit::RelativeHeight(-0.5)), y: Align::Center(), width: UiSize::Size(UIUnit::RelativeHeight(1.0)), height: UiSize::Size(UIUnit::RelativeHeight(1.0)), color, border_color: color, border: 0.0, corner: UIUnit::RelativeHeight(0.5) }
    }

    pub const fn new(x: Align, y: Align, width: UiSize, height: UiSize, color: RGB, border_color: RGB, border: f32, corner: UIUnit) -> Self {
        Self { x, y, width, height, color, border_color, border, corner }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self { x: Align::Left(UIUnit::Zero()), y: Align::Top(UIUnit::Zero()), width: UiSize::Size(UIUnit::Pixel(200.0)), height: UiSize::Size(UIUnit::Pixel(100.0)), color: RGB::WHITE, border_color: RGB::BLACK, border: 1.0, corner: UIUnit::Pixel(5.0) }
    }
}