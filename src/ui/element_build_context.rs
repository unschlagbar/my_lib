use std::ptr::null;

use crate::primitives::Vec2;

use super::{Font, RawUiElement};

pub struct BuildContext {
    pub parent_size: Vec2,
    pub parent_pos: Vec2,
    pub start_pos: Vec2,
    pub parent: *const RawUiElement,
    pub order: u16,
    pub font: *const Font,
}

impl BuildContext {
    pub fn default(font: &Font, parent_size: Vec2) -> Self {
        Self { parent_size, parent_pos: Vec2::default(), start_pos: Vec2::default(), parent: null(), order: 0, font: font as _ }
    }

    pub fn new_from(context: &Self, parent_size: Vec2, parent_pos: Vec2, parent: *const RawUiElement) -> Self {
        Self { parent_size, parent_pos, start_pos: Vec2::default(), parent, order: 0, font: context.font }
    }
}