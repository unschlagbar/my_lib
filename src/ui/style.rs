use crate::primitives::Vec2;
use super::UiUnit;

#[derive(Debug, Clone, Copy)]
pub struct Padding {
    pub left: UiUnit,
    pub right: UiUnit,
    pub top: UiUnit,
    pub bottom: UiUnit,
}

impl Padding {
    pub const fn new(pixel: f32) -> Self {
        Self { left: UiUnit::Px(pixel), right: UiUnit::Px(pixel), top: UiUnit::Px(pixel), bottom: UiUnit::Px(pixel) }
    }

    pub fn x(&self, space: Vec2) -> f32 {
        self.left.pixelx(space) + self.right.pixelx(space)
    }

    pub fn y(&self, space: Vec2) -> f32 {
        self.top.pixely(space) + self.bottom.pixely(space)
    }

    pub fn start(&self, space: Vec2) -> Vec2 {
        Vec2::new(self.left.pixelx(space), self.top.pixely(space))
    }

    pub const fn zero() -> Self {
        Self { left: UiUnit::Zero, right: UiUnit::Zero, top: UiUnit::Zero, bottom: UiUnit::Zero }
    }
    
}

impl Default for Padding {
    fn default() -> Self {
        Self::zero()
    }
    
}