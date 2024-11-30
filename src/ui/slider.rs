use std::ptr::null;

use crate::graphics::formats::RGB;

use super::{RawUiElement, RenderMode, Style, UiElement, UiType};

#[derive(Debug, Clone)]
pub struct Slider {
    pub min_value: f32,
    pub max_value: f32,
    pub value: f32,
    pub step: f32,
    pub padding: f32,
}

impl Slider {
    pub fn new(style: Style, min_value: f32, max_value: f32, value: f32, infill_color: RGB, grip_color: RGB) -> UiElement {
        let mut childs = Vec::with_capacity(2);
        childs.push(UiElement::inline(Style::infill(5.0, infill_color), Vec::with_capacity(0)));
        childs.push(UiElement::new(Style::toggle(grip_color), Vec::with_capacity(0)));
        UiElement { style,
            visible: true,
            mode: RenderMode::Absolute,
            dirty: true,
            parent: null(),
            childs,
            computed: RawUiElement::default(),
            inherit: UiType::Slider(
                Self::slider(min_value, max_value, value),
            )
        }
    }

    pub const fn slider(min_value: f32, max_value: f32, value: f32) -> Self {
        Self { min_value, max_value, value, step: 1.0, padding: 5.0 }
    }
}