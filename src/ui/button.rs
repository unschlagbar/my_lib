use std::ptr::null;
use core::fmt::Debug;

use super::{callback::{self, ErasedFnPointer}, RawUiElement, RenderMode, Style, UiElement, UiState, UiType};

#[derive(Clone)]
pub struct Button {
    pub hover_style: Style,
    pub press_style: Style,
    pub selected: bool,
    pub pressed: bool,
    pub on_press: ErasedFnPointer,
    pub before_press: ErasedFnPointer,
}

impl Button {
    pub const fn new(style: Style, hover_style: Style, press_style: Style, childs: Vec<UiElement>, mode: RenderMode) -> UiElement {
        UiElement {
            style,
            visible: true,
            mode,
            dirty: true,
            childs,
            parent: null(),
            computed: RawUiElement::default(),
            inherit: UiType::Button(Self::button(hover_style, press_style))
        }
    }

    pub const fn button(hover_style: Style, press_style: Style) -> Self {
        Self { hover_style, press_style, selected: false, pressed: false, on_press: ErasedFnPointer::null(), before_press: ErasedFnPointer::null() }
    }

    pub fn on_press<S>(&mut self, struct_pointer: &mut S, fp: fn(&mut S, &mut UiState)) {
        self.on_press = callback::ErasedFnPointer::from_associated(struct_pointer, fp);
    }

    pub fn before_press<S>(&mut self, struct_pointer: &mut S, fp: fn(&mut S, &mut UiState)) {
        self.before_press = callback::ErasedFnPointer::from_associated(struct_pointer, fp);
    }
    
}

impl Debug for Button {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiButton").field("hover_style", &self.hover_style).field("press_style", &self.press_style).field("selected", &self.selected).field("pressed", &self.pressed).finish()
    }
}