
use crate::{graphics::formats::RGBA, primitives::Vec2};

use super::{Align, Overflow, UIUnit, UiSize};


#[derive(Debug, Clone, Copy)]
pub struct  Style {
    pub position: Position,
    pub width: UiSize,
    pub height: UiSize,
    pub color: RGBA,
    pub border_color: RGBA,
    pub border: [f32; 4],
    pub corner: [UIUnit; 4],
    pub padding: Padding,
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Absolute(Absolute),
    Inline(Inline),
}

impl Style {
    pub const FULL: Self = Self {
        position: Position::Absolute(Absolute { align: Align::TopLeft, x: UIUnit::Zero, y: UIUnit::Zero }),
        width: UiSize::Fill,
        height: UiSize::Fill,
        color: RGBA::WHITE,
        border_color: RGBA::BLACK,
        border: [1.0; 4],
        corner: [UIUnit::Pixel(5.0); 4],
        padding: Padding::default(),
    };
    
    pub fn infill(padding: f32, color: RGBA) -> Self {
        //Self::Inline(Inline { margin: [UIUnit::Zero; 4], padding: Padding::new(padding), width: UiSize::Fill, height: UiSize::Fill, color, border_color: color, border: [0.0; 4], corner: [UIUnit::RelativeHeight(0.5); 4], overflow: Overflow::clip() })
        Self {
            position: Position::Inline(Inline { margin: [UIUnit::Zero; 4], overflow: Overflow::clip() }),
            width: UiSize::Fill,
            height: UiSize::Fill,
            color,
            border_color: color,
            border: [0.0; 4],
            corner: [UIUnit::RelativeHeight(0.5); 4],
            padding: Padding::new(padding),
        }
    }

    pub fn label(color: RGBA, corner: UIUnit, width: UiSize, margin: UIUnit) -> Self {
        //Self::Inline(Inline { margin: [margin; 4], padding: Padding::default(), width, height: UiSize::Auto, color, border_color: color, border: [0.0; 4], corner: [corner; 4], overflow: Overflow::clip() })
        Self {
            position: Position::Inline(Inline { margin: [margin; 4], overflow: Overflow::clip() }),
            width,
            height: UiSize::Auto,
            color,
            border_color: color,
            border: [0.0; 4],
            corner: [corner; 4],
            padding: Padding::default(),
        }
    }

    pub fn text(color: RGBA, margin: UIUnit, size: UIUnit) -> Self {
        //Self::Inline(Inline { margin: [margin; 4], padding: Padding::default(), width: UiSize::Auto, height: UiSize::Size(size), color, border_color: color, border: [0.0; 4], corner: [UIUnit::Zero; 4], overflow: Overflow::clip() })
        Self {
            position: Position::Inline(Inline { margin: [margin; 4], overflow: Overflow::clip() }),
            width: UiSize::Auto,
            height: UiSize::Size(size),
            color,
            border_color: color,
            border: [0.0; 4],
            corner: [UIUnit::Zero; 4],
            padding: Padding::default(),
        }
    }

    pub fn toggle(color: RGBA) -> Self {
        //Self::Absolute(Absolute { padding: Padding::default(), align: Align::Left, x: UIUnit::Zero, y: UIUnit::Zero, width: UiSize::Size(UIUnit::RelativeHeight(2.0)), height: UiSize::Size(UIUnit::RelativeHeight(2.0)), color, border_color: RGBA::WHITE, border: [1.0; 4], corner: [UIUnit::RelativeHeight(0.5); 4] })
        Self {
            position: Position::Absolute(Absolute { align: Align::Left, x: UIUnit::Zero, y: UIUnit::Zero }),
            width: UiSize::Size(UIUnit::RelativeHeight(2.0)),
            height: UiSize::Size(UIUnit::RelativeHeight(2.0)),
            color,
            border_color: RGBA::WHITE,
            border: [1.0; 4],
            corner: [UIUnit::RelativeHeight(0.5); 4],
            padding: Padding::default(),
        }
    }

    pub const fn new(align: Align, x: UIUnit, y: UIUnit, width: UiSize, height: UiSize, color: RGBA, border_color: RGBA, border: f32, corner: UIUnit) -> Self {
        //Self::Absolute(Absolute { padding: Padding::default(), align, x, y, width, height, color, border_color, border: [border; 4], corner: [corner; 4] })
        Self {
            position: Position::Absolute(Absolute { align, x, y }),
            width,
            height,
            color,
            border_color,
            border: [border; 4],
            corner: [corner; 4],
            padding: Padding::default(),
        }
    }

    pub const fn inline(margin: UIUnit, color: RGBA, border_color: RGBA, border: f32, corner: UIUnit, width: UiSize, height: UiSize) -> Self {
        //Self::Inline(Inline { margin: [margin; 4], padding: Padding::default(), width, height, color, border_color, border: [border; 4], corner: [corner; 4], overflow: Overflow::clip() })
        Self {
            position: Position::Inline(Inline { margin: [margin; 4], overflow: Overflow::clip() }),
            width,
            height,
            color,
            border_color,
            border: [border; 4],
            corner: [corner; 4],
            padding: Padding::default(),
        }
    }
}

impl Style {

}

impl Default for Style {
    fn default() -> Self {
        Self {
            position: Position::Absolute(Absolute::default()),
            width: UiSize::Auto,
            height: UiSize::Auto,
            color: RGBA::WHITE,
            border_color: RGBA::BLACK,
            border: [1.0; 4],
            corner: [UIUnit::Pixel(5.0); 4],
            padding: Padding::default(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Absolute {
    pub align: Align,
    pub x: UIUnit,
    pub y: UIUnit,
}

#[derive(Debug, Clone, Copy)]
pub struct Inline {
    pub margin: [UIUnit; 4],
    pub overflow: Overflow,
}

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

impl Padding {
    pub const fn new(padding: f32) -> Self {
        Self { left: padding, right: padding, top: padding, bottom: padding }
    }

    pub fn x(&self) -> f32 {
        self.left + self.right
    }

    pub fn y(&self) -> f32 {
        self.top + self.bottom
    }

    pub fn start(&self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    pub const fn default() -> Self {
        Self { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }
    }
    
}

impl Default for Padding {
    fn default() -> Self {
        Self { left: 0.0, right: 0.0, top: 0.0, bottom: 0.0 }
    }
    
}

impl Absolute {

}

impl Inline {

}

impl Default for Absolute {
    fn default() -> Self {
        Self { align: Align::TopLeft, x: UIUnit::Zero, y: UIUnit::Zero }
    }
}

impl Default for Inline {
    fn default() -> Self {
        Self { margin: [UIUnit::Zero; 4], overflow: Overflow::clip() }
    }
}