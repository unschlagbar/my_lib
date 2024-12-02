use std::ptr::null;

use super::{BasicAlign, RawUiElement, Style, UiElement, UiType};


#[derive(Debug, Clone)]
pub struct DragBox {
    pub grip_height: f32,
    pub snap_strenght: f32,
    pub align: BasicAlign,
    pub pressed: bool,
    pub move_parrent: bool,
}

impl DragBox {
    pub const fn new(style: Style) -> UiElement {
        UiElement {
            style,
            visible: true,
            dirty: true,
            parent: null(),
            childs: vec![],
            computed: RawUiElement::default(),
            inherit: UiType::DragBox(Self { grip_height: 20.0, snap_strenght: 50.0, align: BasicAlign::Center, pressed: false, move_parrent: true }),
        }
    }
}