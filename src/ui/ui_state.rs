
use cgmath::Vector2;
use crate::{graphics::{UiInstance, VertexUi}, primitives::Vec2};
use super::{raw_ui_element::UiEvent, BuildContext, Font, Style, UiElement, UiType};

#[derive(Debug)]
pub struct UiState {
    elements: Vec<UiElement>,
    //styles: Vec<Style>,
    pub selected: Option<*const UiElement>,
    pub pressed: Option<*const UiElement>,
    pub cursor_pos: Vec2,
    pub font: Font,
    pub visible: bool,
    pub dirty: bool,
}

impl UiState {
    pub const fn get_vertices() -> [VertexUi; 4] {
        let vertices = [
            VertexUi { uv: Vector2 { x: 0.0, y: 0.0 }},
            VertexUi { uv: Vector2 { x: 1.0, y: 0.0 }},
            VertexUi { uv: Vector2 { x: 0.0, y: 1.0 }},
            VertexUi { uv: Vector2 { x: 1.0, y: 1.0 }}
        ];
        vertices
    }

    pub fn create(elements: Vec<UiElement>, _styles: Vec<Style>, visible: bool) -> UiState {
        UiState { visible, elements, dirty: false, cursor_pos: Vec2::default(), selected: None, pressed: None, font: Font::parse_from_bytes(include_bytes!("C:/Dev/vudeljump/font/std1.fef")) }
    }

    pub fn font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn build(&mut self, ui_size: Vec2) {

        self.selected = None;
        self.pressed = None;

        let mut build_context = BuildContext::default(&self.font, ui_size);
        
        for element in &mut self.elements {
            element.build(&mut build_context);
            build_context.order += 1;
        }
    }

    pub fn get_instaces(&mut self, ui_size: Vec2) -> Vec<UiInstance> {
        println!("ff");
        let mut instances = Vec::new();

        if !self.visible || self.elements.len() == 0 {
            instances.push(UiElement::default().computed.to_instance());
            return instances;
        }

        for raw_e in &mut self.elements {
            raw_e.get_instances(&mut instances, ui_size, &self.font);
        }

        self.dirty = false;

        instances
    }

    pub unsafe fn get_element_mut(&mut self, root: Vec<usize>) -> Option<&mut UiElement> {
        let mut h = &mut self.elements[*root.first()?];
        for i in 1..root.len() {
            h = &mut h.childs[*root.get(i)?];
        }

        Some(h)
    }

    pub fn get_element(&self, root: Vec<usize>) -> Option<&UiElement> {
        let mut h = &self.elements[*root.first()?];
        for i in 1..root.len() {
            h = &h.childs[*root.get(i)?];
        }

        Some( &h )
    }

    pub fn update_cursor(&mut self, ui_size: Vec2, cursor_pos: Vec2, event: UiEvent) -> u8 {

        let mut bol = 0;

        let ui = unsafe { &mut *(self as *const UiState).cast_mut() };

        for i in &mut self.elements {
            let result = i.update_cursor(ui, ui_size, Vec2::default(), cursor_pos, event);
            if result != 0 {
                bol = result;
                break;
            }
        }

        //Not old event
        if bol != 2 {
            if let Some(selected) = self.selected {
                let selected = unsafe { &mut *selected.cast_mut() };

                //No event
                if bol == 0 {
                    self.selected = None;
                    match &mut selected.inherit {
                        UiType::Button(button) => {
                            if button.selected {
                                button.selected = false;
                                selected.dirty = true;
                                self.dirty = true;
                                return 1;
                            }
                        }
                        _ => todo!()
                    }
                }
            } 
            
            if let Some(pressed) = self.pressed {
                match event {
                    UiEvent::Release => {
                        let element = unsafe { &mut *pressed.cast_mut() };
                        let element_copy = unsafe { & *pressed };
                        match &mut element.inherit {
                            UiType::Button(button) => {
                                button.pressed = false;
                                self.pressed = None;
                                element.dirty = true;
                                bol = 1;
                                if element_copy.is_in(cursor_pos) {
                                    button.on_press.call(ui);
                                }
                            },
                            UiType::DragBox(dragbox) => {
                                dragbox.pressed = false;
                                self.pressed = None;
                                println!("released");
                            }
                            _ => ()
                        }
                    },
                    UiEvent::Move => {
                        let element = unsafe { &mut *pressed.cast_mut() };
                        match &mut element.inherit {
                            UiType::DragBox(_) => {
                                match element.style {
                                    Style::Inline(_) if !element.parent.is_null() => {
                                        let parent = unsafe { &mut *(element.parent as *mut UiElement) };
                                        parent.computed.pos += cursor_pos - self.cursor_pos;
                                    },
                                    _ => element.computed.pos += cursor_pos - self.cursor_pos
                                }
                                bol = 1;
                            }
                            _ => ()
                        }
                    }
                    _ => ()
                }
            }
        }
        //New event
        if bol == 1 { self.dirty = true }
        self.cursor_pos = cursor_pos;
        bol
    }

}