
use super::{Button, CheckBox, DragBox, Slider, Text, TextInput, UiImage};

#[derive(Debug, Clone)]
pub enum UiType {
    Block(),
    InlineText(),
    Text(Text),
    Button(Button),
    Image(UiImage),
    TextInput(TextInput),
    DragBox(DragBox),
    CheckBox(CheckBox),
    Slider(Slider),
}