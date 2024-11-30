
use super::{style::Style, UiType};

#[derive(Debug)]
pub struct UiDiv {
    pub style: Style,
    pub childs: Vec<UiType>,
    pub mode: u8,
    pub visible: bool,
    pub dragable: u8
}

impl UiDiv {
    pub const fn square(style: Style, childs: Vec<UiType>) -> Self {
        Self { style, childs, visible: true, dragable: 0, mode: 0 }
    }
}