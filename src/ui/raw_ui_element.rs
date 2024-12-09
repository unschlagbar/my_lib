use crate::{graphics::{formats::Color, UiInstance}, primitives::Vec2};

use super::Style;


#[derive(Clone, Debug)]
pub struct RawUiElement {
    pub color: Color,
    pub border_color: Color,
    pub border: f32,
    pub pos: Vec2,
    pub size: Vec2,
    pub corner: f32,
    pub mode: u32,
    pub order: u16,
}

impl RawUiElement {

    pub const fn new(pos: Vec2, size: Vec2, color: Color, border_color: Color, border: f32, corner: f32, order: u16, mode: u32) -> Self {
        Self { pos, size , color, border_color, border, corner, order, mode}
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

    pub fn set_new_style(&mut self, new_style: Style) {
        let _ = new_style;
        todo!()
    }

    pub const fn default() -> Self {
        Self { pos: Vec2::zero(), size: Vec2::zero(), color: Color::ZERO, border_color: Color::ZERO, border: 0.0, corner: 0.0, order: 0, mode: 0 }
    }

    
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum UiEvent {
    Press,
    Release,
    Move,
}

pub trait UiCallback {
    fn call(&self);
}

impl<F: 'static + Fn()> UiCallback for F {
    fn call(&self) {
        self();
    }
}