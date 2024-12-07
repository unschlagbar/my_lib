
use crate::graphics::formats::RGBA;

use super::{Align, UIUnit, UiSize};


#[derive(Debug, Clone)]
pub enum Style {
    Absolute(Absolute),
    Inline(Inline),
}

impl Style {
    pub const FULL: Self = Self::Inline( Inline {margin: [UIUnit::Zero; 4], padding: [UIUnit::Zero; 4], width: UiSize::Size(UIUnit::Relative(1.0)), height: UiSize::Size(UIUnit::Relative(1.0)), color: RGBA::WHITE, border_color: RGBA::BLACK, border: [1.0; 4], corner: [UIUnit::Zero; 4]} );
    
    pub fn infill(padding: f32, color: RGBA) -> Self {
        Self::Inline(Inline { margin: [UIUnit::Zero; 4], padding: [UIUnit::Pixel(padding); 4], width: UiSize::Fill, height: UiSize::Fill, color, border_color: color, border: [0.0; 4], corner: [UIUnit::RelativeHeight(0.5); 4] })
    }

    pub fn label(color: RGBA, corner: UIUnit, width: UiSize) -> Self {
        Self::Inline(Inline { margin: [UIUnit::Zero; 4], padding: [UIUnit::Zero; 4], width, height: UiSize::Auto, color, border_color: color, border: [0.0; 4], corner: [corner; 4] })
    }

    pub fn text(color: RGBA, margin: UIUnit, size: UIUnit) -> Self {
        Self::Inline(Inline { margin: [margin; 4], padding: [UIUnit::Zero; 4], width: UiSize::Auto, height: UiSize::Size(size), color, border_color: color, border: [0.0; 4], corner: [UIUnit::Zero; 4] })
    }

    pub fn toggle(color: RGBA) -> Self {
        Self::Absolute(Absolute { x: Align::Left(UIUnit::RelativeHeight(-0.5)), y: Align::Center(), width: UiSize::Size(UIUnit::RelativeHeight(1.0)), height: UiSize::Size(UIUnit::RelativeHeight(1.0)), color, border_color: color, border: [0.0; 4], corner: [UIUnit::RelativeHeight(0.5); 4] })
    }

    pub const fn new(x: Align, y: Align, width: UiSize, height: UiSize, color: RGBA, border_color: RGBA, border: f32, corner: UIUnit) -> Self {
        Self::Absolute(Absolute { x, y, width, height, color, border_color, border: [border; 4], corner: [corner; 4] })
    }

    pub const fn inline(margin: UIUnit, color: RGBA, border_color: RGBA, border: f32, corner: UIUnit, width: UiSize, height: UiSize) -> Self {
        Self::Inline(Inline { margin: [margin; 4], padding: [UIUnit::Zero; 4], width, height, color, border_color, border: [border; 4], corner: [corner; 4] })
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::Inline(Inline { margin: [UIUnit::Zero; 4], padding: [UIUnit::Zero; 4], width: UiSize::Auto, height: UiSize::Auto, color: RGBA::WHITE, border_color: RGBA::BLACK, border: [1.0; 4], corner: [UIUnit::Pixel(5.0); 4] })
    }
}

#[derive(Debug, Clone)]
pub struct Absolute {
    pub x: Align,
    pub y: Align,
    pub width: UiSize,
    pub height: UiSize,
    pub color: RGBA,
    pub border_color: RGBA,
    pub border: [f32; 4],
    pub corner: [UIUnit; 4],
}
#[derive(Debug, Clone)]
pub struct Inline {
    pub margin: [UIUnit; 4],
    pub padding: [UIUnit; 4],
    pub width: UiSize,
    pub height: UiSize,
    pub color: RGBA,
    pub border_color: RGBA,
    pub border: [f32; 4],
    pub corner: [UIUnit; 4],
}