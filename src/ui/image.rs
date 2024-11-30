use std::ptr::null;

use super::{RawUiElement, RenderMode, Style, UiElement};



#[derive(Debug, Clone)]
pub struct UiImage {
    pub index: u8,
}

impl UiImage {
    pub fn new(style: Style, index: u8) -> UiElement {
        UiElement { style,
            visible: true, 
            mode: RenderMode::Absolute,
            dirty: true,
            parent: null(),
            childs: Vec::with_capacity(0),
            computed: RawUiElement::default(),
            inherit: super::UiType::Image(Self { index })
        }
    }
}