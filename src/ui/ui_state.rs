
use std::ptr::null_mut;

use crate::{graphics::UiInstance, primitives::Vec2};
use super::{raw_ui_element::UiEvent, BuildContext, Font, Style, UiElement, UiType};

#[derive(Debug)]
pub struct UiState {
    elements: Vec<UiElement>,
    //styles: Vec<Style>,
    pub selected: *mut UiElement,
    pub pressed: *mut UiElement,
    pub cursor_pos: Vec2,
    pub font: Font,
    pub visible: bool,
    pub dirty: bool,
}

impl UiState {

    pub fn create(elements: Vec<UiElement>, _styles: Vec<Style>, visible: bool) -> UiState {
        UiState { visible, elements, dirty: false, cursor_pos: Vec2::default(), selected: null_mut(), pressed: null_mut(), font: Font::parse_from_bytes(include_bytes!("C:/Dev/vudeljump/font/std1.fef")) }
    }

    pub fn font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn build(&mut self, ui_size: Vec2) {

        self.selected = null_mut();
        self.pressed = null_mut();

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

        //0 = no event
        //1 = no event break
        //2 = old event
        //3 = new event
        let mut bol = 0;

        let ui = unsafe { &mut *(self as *const UiState).cast_mut() };

        for i in &mut self.elements {
            let result = i.update_cursor(ui, ui_size, Vec2::default(), cursor_pos, event);
            if result > 0 {
                bol = result;
                break;
            }
        }

        //Not old event
        if bol != 2 {
            if !self.selected.is_null() && bol < 2 {
                let selected = unsafe { &mut *self.selected };
                println!("{}", bol);

                self.selected = null_mut();
                match &mut selected.inherit {
                    UiType::Button(button) => {
                        if button.selected {
                            button.selected = false;
                            selected.dirty = true;
                            self.dirty = true;
                            return 3;
                        }
                    },
                    UiType::CheckBox(checkbox) => {
                        if checkbox.selected {
                            checkbox.selected = false;
                            selected.dirty = true;
                            self.dirty = true;
                            return 3;
                        }
                    }
                    _ => todo!()
                }
            } 
            if !self.pressed.is_null() {
                let element = unsafe { &mut *self.pressed };
                let is_in = element.is_in(cursor_pos);
                match event {
                    UiEvent::Move => {
                        match &element.inherit {
                            UiType::DragBox(_) => {
                                match element.style {
                                    Style::Inline(_) if !element.parent.is_null() => {
                                        let parent = unsafe { &mut *element.parent };
                                        parent.move_computed(cursor_pos - self.cursor_pos);
                                    },
                                    _ => element.computed.pos += cursor_pos - self.cursor_pos,
                                }
                                bol = 3;
                            }
                            _ => ()
                        }
                    },
                    UiEvent::Release => {
                        match &mut element.inherit {
                            UiType::Button(button) => {
                                button.pressed = false;
                                self.pressed = null_mut();
                                element.dirty = true;
                                bol = 3;
                                if is_in {
                                    button.on_press.call(ui);
                                }
                            },
                            UiType::CheckBox(checkbox) => {
                                checkbox.pressed = false;
                                self.pressed = null_mut();
                                element.dirty = true;
                                bol = 3;
                                if is_in {
                                    if checkbox.enabled {
                                        checkbox.on_disable.call(ui);
                                    } else {
                                        checkbox.on_enable.call(ui);
                                    }
                                    checkbox.enabled = !checkbox.enabled;
                                }
                            },
                            UiType::DragBox(dragbox) => {
                                dragbox.pressed = false;
                                self.pressed = null_mut();
                                println!("released");
                            },
                            _ => ()
                        }
                    },
                    _ => (),
                }
            }
        }
        //New event
        if bol == 3 { self.dirty = true }
        self.cursor_pos = cursor_pos;
        bol
    }

}