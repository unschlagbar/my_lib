use crate::{graphics::{formats::Color, UiInstance}, primitives::Vec2};

#[derive(Clone, Debug)]
pub struct RawUiElement {
    pub color: Color,
    pub border_color: Color,
    pub border: f32,
    pub view: Vec2,
    pub pos: Vec2,
    pub size: Vec2,
    pub corner: f32,
    pub mode: u32,
}

impl RawUiElement {

    pub const fn new(pos: Vec2, size: Vec2, color: Color, border_color: Color, border: f32, view: Vec2, corner: f32, mode: u32) -> Self {
        Self { pos, size , color, border_color, border, corner, view, mode }
    }

    #[inline(always)]
    pub fn to_instance(&self) -> UiInstance {
        UiInstance { 
            color: self.color,
            border_color: self.border_color,
            border: self.border,
            x: self.pos.x.floor(),
            y: self.pos.y.floor(),
            width: self.size.x.floor(),
            height: self.size.y.floor(),
            corner: self.corner,
            mode: self.mode
        }
    }
}

impl Default for RawUiElement {
    fn default() -> Self {
        Self { pos: Vec2::zero(), size: Vec2::zero(), view: Vec2::zero(), color: Color::ZERO, border_color: Color::ZERO, border: 0.0, corner: 0.0, mode: 0 }
    }
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UiEvent {
    Press,
    Release,
    Move,
}