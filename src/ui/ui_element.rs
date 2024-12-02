use std::ptr::null;

use crate::{graphics::UiInstance, primitives::Vec2};

use super::{Align, BuildContext, Font, RawUiElement, Style, UiEvent, UiState, UiType};

#[derive(Clone, Debug)]
pub struct UiElement {
    pub style: Style,
    pub visible: bool,
    pub dirty: bool,
    pub parent: *const UiElement,
    pub childs: Vec<UiElement>,
    pub computed: RawUiElement,
    pub inherit: UiType
}

impl UiElement {

    pub const fn new(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null() }
    }

    pub const fn inline(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null() }
    }

    #[inline(always)]
    pub fn build(&mut self, context: &mut BuildContext) {
        if !self.visible {
            self.computed.order = context.order;
            self.dirty = false;
            return;
        }

        let self_copy = unsafe { &mut *(self as *mut _) };

        match &self.inherit {
            UiType::Text(text) => {
                text.build(self_copy, context);
            },
            UiType::Slider(slider) => {
                slider.build(self_copy, context);
            },
            UiType::Image(image) => {
                image.build(self_copy, context);
            }
            _ => {

                let size;
                let mut pos;

                match &self.style {
                    Style::Absolute(absolute) => {
                        size = Vec2::new(
                            absolute.width.width(context.parent_size),
                            absolute.height.height(context.parent_size)
                        );

                        pos = Vec2::new(
                            match absolute.x {
                                Align::Left(unit) => {
                                    unit.pixelx(context.parent_size)
                                },
                                Align::Right(unit) => {
                                    -unit.pixelx(context.parent_size) - size.x
                                },
                                Align::Center() => {
                                    (context.parent_size.x - size.x) * 0.5
                                },
                                _ => panic!()
                            },
                            match absolute.y {
                                Align::Top(unit) => {
                                    unit.pixely(context.parent_size)
                                },
                                Align::Bottom(unit) => {
                                    context.parent_size.y - unit.pixely(context.parent_size) - size.y
                                },
                                Align::Center() => {
                                    (context.parent_size.y - size.y) * 0.5 
                                },
                                _ => panic!()
                            },
                        );

                        self.computed.color = absolute.color.as_color();
                        self.computed.border_color = absolute.border_color.as_color();
                        self.computed.border = absolute.border[0];
                        self.computed.corner = absolute.corner[0].pixelx(size);
                    },
                    Style::Inline(inline) => {
                        size = Vec2::new(
                            inline.width.width(context.parent_size),
                            inline.height.height(context.parent_size)
                        );

                        pos = Vec2::new(0.0, 0.0);

                        if context.parent_size.x - context.start_pos.x - context.parent_pos.x >= size.x {
                            pos.x += context.start_pos.x;
                            context.start_pos.x += size.x;
                        } else {
                            pos.y += context.start_pos.y;
                            context.start_pos.y += size.y;
                        }

                        self.computed.color = inline.color.as_color();
                        self.computed.border_color = inline.border_color.as_color();
                        self.computed.border = inline.border[0];
                        self.computed.corner = inline.corner[0].pixelx(size);
                    }
                }

                pos += context.parent_pos;

                {
                    self.computed.pos = pos;
                    self.computed.size = size;
                    self.computed.order = context.order;
                }

                let mut context = BuildContext::new_from(context, size, pos, &self.computed as _);

                for element in &mut self.childs {
                    element.build(&mut context);
                    context.order += 1;
                }
                self.dirty = false;
            }
        }
    }

    #[inline(always)]
    pub fn get_instances(&mut self, instances: &mut Vec<UiInstance>, ui_size: Vec2, font: &Font) {

        if self.dirty {
            if self.parent.is_null() {
                self.rebuild(ui_size, Vec2::default(), font);
            }
        }

        if !self.visible {
            for child in &mut self.childs {
                child.get_instances(instances, ui_size, font);
            }
            return;
        }

        instances.push(self.computed.to_instance());

        if let UiType::Text(text) = &self.inherit {
            instances.extend_from_slice(text.comp_text.as_slice());
        }

        for child in &mut self.childs {
            child.get_instances(instances, ui_size, font);
        }
    }

    #[inline(always)]
    pub fn get_offset(&self) -> Vec2 {
        if self.parent.is_null() {
            return Vec2::default();
        } else {
            let parent = unsafe { &*self.parent };
            let offset;

            if self.computed.order > 0 {
                let child = &parent.childs[self.computed.order as usize - 1];
                offset = child.computed.pos - parent.computed.pos + child.computed.size;
            } else {
                offset = Vec2::default();
            }

            offset
        }
    }

    #[inline(always)]
    pub fn rebuild(&mut self, parent_size: Vec2, parent_pos: Vec2, font: &Font) {

        let style: &Style = match &self.inherit {
            UiType::Button(button) => {
                if button.pressed {
                    &button.press_style
                } else if button.selected {
                    &button.hover_style
                } else {
                    &self.style
                }
            }
            _ => &self.style,
        };

        match style {
            Style::Absolute(absolute) => {
                self.computed.size = Vec2::new(
                    absolute.width.width(parent_size),
                    absolute.height.height(parent_size)
                );

                self.computed.pos = Vec2::new(
                    match absolute.x {
                        Align::Left(unit) => {
                            unit.pixelx(parent_size)
                        },
                        Align::Right(unit) => {
                            -unit.pixelx(parent_size) - self.computed.size.x
                        },
                        Align::Center() => {
                            (parent_size.x - self.computed.size.x) * 0.5
                        },
                        _ => panic!()
                    },
                    match absolute.y {
                        Align::Top(unit) => {
                            unit.pixely(parent_size)
                        },
                        Align::Bottom(unit) => {
                            parent_size.y - unit.pixely(parent_size) - self.computed.size.y
                        },
                        Align::Center() => {
                            (parent_size.y - self.computed.size.y) * 0.5 
                        },
                        _ => panic!()
                    },
                );

                self.computed.color = absolute.color.as_color();
                self.computed.border_color = absolute.border_color.as_color();
                self.computed.border = absolute.border[0];
                self.computed.corner = absolute.corner[0].pixelx(self.computed.size);
            },
            Style::Inline(inline) => {
                self.computed.size = Vec2::new(
                    inline.width.width(parent_size),
                    inline.height.height(parent_size)
                );

                self.computed.pos = Vec2::new(0.0, 0.0);

                let start_pos = self.get_offset();

                if parent_size.x - start_pos.x - parent_pos.x >= self.computed.size.x {
                    self.computed.pos.x += start_pos.x;
                } else {
                    self.computed.pos.y += start_pos.y;
                }

                self.computed.color = inline.color.as_color();
                self.computed.border_color = inline.border_color.as_color();
                self.computed.border = inline.border[0];
                self.computed.corner = inline.corner[0].pixelx(self.computed.size);
            }
        };

        if let UiType::Text(text) = &mut self.inherit {
            text.build_text(self.computed.size, self.computed.pos, font);
        }

        self.dirty = false
    }

    #[inline]
    pub fn is_in(&self, pos: Vec2) -> bool {
        if self.computed.pos < pos {
            if self.computed.pos.x + self.computed.size.x > pos.x && self.computed.pos.y + self.computed.size.y > pos.y {
                return true;
            }
        }
        false
    }

    #[allow(unused)]
    #[inline]
    pub fn update_cursor(&mut self, ui: &mut UiState, parent_size: Vec2, parent_pos: Vec2, cursor_pos: Vec2, ui_event: UiEvent) -> u8 {
        //0 = no event
        //1 = new event
        //2 = old event

        if !self.visible {
            return 0;
        }

        let computed = &self.computed;

        if computed.pos < cursor_pos {

            if computed.pos.x + computed.size.x > cursor_pos.x && computed.pos.y + computed.size.y > cursor_pos.y {

                for child in &mut self.childs {
                    let result = child.update_cursor(ui, computed.size, computed.pos, cursor_pos, ui_event);
                    if result != 0 { return result };
                }

                let self_ptr = self as *const UiElement;

                match ui_event {
                    UiEvent::Press => {
                        match &mut self.inherit {
                            UiType::Button(button) => {
                                button.pressed = true;
                                ui.pressed = Some(self_ptr);
                                button.before_press.call(ui);
                                self.dirty = true;
                            },
                            UiType::DragBox(dragbox) => {
                                dragbox.pressed = true;
                                ui.pressed = Some(self_ptr);
                            }
                            _ => ()
                        }
                    },
                    UiEvent::Release => (),
                    UiEvent::Move => {
                        match &mut self.inherit {
                            UiType::Button(button) => {
                                if !button.selected {
                                    button.selected = true;

                                    self.dirty = true;

                                    if let Some(s) = ui.selected {
                                        let raw_ref = unsafe { (s as *mut UiElement).as_mut().unwrap_unchecked() };
                                        match unsafe { &mut raw_ref.inherit } {
                                            UiType::Button(b) => {
                                                b.selected = false;
                                                self.dirty = true;
                                            },
                                            _ => ()
                                        }
                                    }
                                    ui.selected = Some(self);
                                } else {
                                    return 2;
                                }
                            },
                            UiType::DragBox(_dragbox) => (),
                            _ => {
                                return 0;
                            }
                        }
                    },
                };
                return 1;
            }
        }
        0
    }

    #[inline]
    pub fn set_dirty(&self) {
        unsafe { (self as *const UiElement as *mut UiElement).as_mut().unwrap_unchecked().dirty = true };
    }

    pub fn add_child(&mut self, mut child: Self) {
        child.parent = self as _;
        self.childs.push(child);
    }
}

impl Default for UiElement {
    fn default() -> Self {
        Self { style: Default::default(), childs: Default::default(), visible: true, dirty: false, computed: RawUiElement::default(), parent: null(), inherit: UiType::Block() }
    }
}