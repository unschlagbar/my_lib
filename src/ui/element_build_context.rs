use std::ptr::null;

use crate::primitives::Vec2;

use super::{style::Inline, Font, RawUiElement};

pub struct BuildContext {
    pub parent_size: Vec2,
    pub parent_pos: Vec2,
    pub line_offset: f32,
    pub start_pos: Vec2,
    pub parent: *const RawUiElement,
    pub order: u16,
    pub font: *const Font,
}

impl BuildContext {
    pub fn default(font: &Font, parent_size: Vec2) -> Self {
        Self { parent_size, parent_pos: Vec2::default(), line_offset: 0.0, start_pos: Vec2::default(), parent: null(), order: 0, font: font as _ }
    }

    pub fn new_from(context: &Self, parent_size: Vec2, parent_pos: Vec2, parent: *const RawUiElement) -> Self {
        Self { parent_size, parent_pos, line_offset: 0.0, start_pos: Vec2::default(), parent, order: 0, font: context.font }
    }

    #[inline]
    pub fn fits_in_line(&mut self , inline: &Inline, pos: &mut Vec2, size: &mut Vec2) -> bool {
        if self.parent_size.x - self.start_pos.x >= size.x {
            println!("sdfsd{:?}", self.start_pos);
            *pos += self.start_pos;

            self.line_offset = self.line_offset.max(size.y + inline.margin[1].pixely(self.parent_size) + inline.margin[3].pixely(self.parent_size));
            self.start_pos.x += size.x + inline.margin[0].pixelx(self.parent_size) + inline.margin[2].pixelx(self.parent_size);

            return true;

        } else {
            self.start_pos.y += self.line_offset;
            pos.y += self.start_pos.y;
            self.line_offset = size.y + inline.margin[1].pixely(self.parent_size) + inline.margin[3].pixely(self.parent_size);
            self.start_pos.x = size.x + inline.margin[0].pixelx(self.parent_size) + inline.margin[2].pixelx(self.parent_size);

            return false;
        }
    }
}