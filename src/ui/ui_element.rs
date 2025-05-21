use std::rc::Rc;

use crate::{graphics::UiInstance, primitives::Vec2};

use super::{AbsoluteLayout, BuildContext, Container, ElementType, Font, RawUiElement, UiEvent, UiState};

pub trait Element {
    fn build(&mut self, context: &mut BuildContext);
    fn instance(&self) -> UiInstance;
    fn childs(&mut self) -> &mut[UiElement];
    fn add_child(&mut self, child: UiElement);
    fn comp(&mut self) -> &mut RawUiElement;
}

pub trait TypeConst {
    const ELEMENT_TYPE: ElementType;
}

pub struct UiElement {
    pub id: u32,
    pub typ: ElementType,
    pub dirty: bool,
    pub visible: bool,
    pub size: Vec2,
    pub pos: Vec2,
    pub parent: *mut UiElement,
    pub element: Box<dyn Element>
}

impl UiElement {

    pub unsafe fn downcast<'a, T: Element>(&'a self) -> &'a T {
        let raw: *const dyn Element = &*self.element as *const dyn Element;
        unsafe { &*(raw as *const T) }
    }

    pub unsafe fn downcast_mut<'a, T: Element>(&'a mut self) -> &'a mut T {
        let raw: *mut dyn Element = &mut*self.element as *mut dyn Element;
        unsafe { &mut*(raw as *mut T) }
    }

    pub fn parent(&mut self) -> &mut UiElement {
        unsafe { &mut *self.parent }
    }

    pub fn build(&mut self, context: &mut BuildContext) {
        match &self.typ {
            ElementType::Block => {
                let div: &mut Container = unsafe { Self::downcast_mut(self) };
                div.build(context);
                self.dirty = false;
            },
            ElementType::AbsoluteLayout => {
                let div: &mut AbsoluteLayout = unsafe { Self::downcast_mut(self) };
                div.build(context);
                self.dirty = false;
            },
            _ => unimplemented!()
        }
    }

    #[inline(always)]
    pub fn get_instances(&mut self, instances: &mut Vec<UiInstance>, ui_size: Vec2, font: &Font) {

        if self.dirty {
            let (_size, _pos) = if !self.parent.is_null() {
                let parent = self.parent();
                (parent.size, parent.pos)
            } else {
                (ui_size, Vec2::default())
            };
            //self.rebuild(size, pos, font);
        }

        if !self.visible {
            return;
        }
        
        if self.typ == ElementType::Text {
            println!("todo");
            //instances.extend_from_slice(text.comp_text.as_slice());
        } else {
            instances.push(self.element.instance());
        }

        for child in self.element.childs() {
            child.get_instances(instances, ui_size, font);
        }
    }

    #[inline(always)]
    pub fn get_offset(&mut self) -> Vec2 {
        let mut offset = Vec2::default();
        if !self.parent.is_null() {
            let id = self.id;
            let parent = self.parent();
            let childs = parent.element.childs();

            for child in childs {
                if child.id == id {
                    break;
                }
                offset = child.pos - parent.pos + child.size;
            }
        }
        offset
    }

    #[inline(always)]
    pub fn move_computed(&mut self, amount: Vec2) {
        for child in self.element.childs() {
            child.move_computed(amount);
        }
        self.pos += amount;

        if self.typ == ElementType::Text {
            todo!()
            //for raw in &mut text.comp_text {
            //    raw.x += amount.x;
            //    raw.y += amount.y;
            //}
        }
    }

    #[inline(always)]
    pub fn move_computed_absolute(&mut self, pos: Vec2) {
        for child in self.element.childs() {
            child.move_computed_absolute(pos);
        }
        self.pos = pos;
    }

    #[inline]
    pub fn is_in(&self, pos: Vec2) -> bool {
        if self.pos < pos {
            if self.pos.x + self.size.x > pos.x && self.pos.y + self.size.y > pos.y {
                return true;
            }
        }
        false
    }

    #[allow(unused)]
    #[inline]
    pub fn update_cursor(&mut self, ui: &mut UiState, parent_size: Vec2, parent_pos: Vec2, cursor_pos: Vec2, ui_event: UiEvent) -> u8 {
        //0 = no event
        //1 = no event break
        //2 = old event
        //3 = new event

        if !self.visible {
            return 0;
        }

        let (self_size, self_pos) = (self.size, self.pos);

        if self_pos < cursor_pos {

            if self_pos.x + self_size.x > cursor_pos.x && self_pos.y + self_size.y > cursor_pos.y {

                for child in self.element.childs() {
                    let result = child.update_cursor(ui, self_size, self_pos, cursor_pos, ui_event);
                    if result > 0 { return result };
                }

                match ui_event {
                    UiEvent::Press => {
                        todo!();
                    },
                    UiEvent::Release => {
                        todo!();
                    },
                    UiEvent::Move => {
                        todo!();
                    },
                };
                return 3;
            }
        }
        0
    }

    pub fn add_to_parent(mut self, parent: &mut UiElement) {
        self.parent = parent as *mut UiElement;
        parent.add_child(self);
    }

    #[inline]
    pub fn get_mut(this: &mut Rc<UiElement>) -> Option<&mut UiElement> {
        Rc::get_mut(this)
    }

    #[inline]
    pub fn set_dirty(&self) {
        unsafe { (self as *const UiElement as *mut UiElement).as_mut().unwrap_unchecked().dirty = true };
    }

    #[inline]
    pub fn add_child(&mut self, mut child: UiElement) {
        child.parent = self as *mut UiElement;
        self.element.add_child(child);
    }
}