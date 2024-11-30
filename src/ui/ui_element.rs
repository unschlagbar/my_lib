use std::ptr::null;

use crate::{graphics::{formats::Color, UiInstance}, primitives::Vec2};

use super::{Align, BuildContext, Font, RawUiElement, RenderMode, Style, Text, UiEvent, UiState, UiType};

#[derive(Clone, Debug)]
pub struct UiElement {
    pub style: Style,
    pub visible: bool,
    pub mode: RenderMode,
    pub dirty: bool,
    pub parent: *const UiElement,
    pub childs: Vec<UiElement>,
    pub computed: RawUiElement,
    pub inherit: UiType
}

impl UiElement {

    pub const fn new(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, mode: RenderMode::Absolute, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null() }
    }

    pub const fn inline(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, mode: RenderMode::Inline, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null() }
    }

    #[inline(always)]
    pub fn build(&mut self, context: &mut BuildContext) {
        if !self.visible {
            self.computed.order = context.order;
            self.dirty = false;
            return;
        }

        match &self.inherit {
            UiType::Text(text) => {

                let size = Vec2::new(
                    self.style.width.width(context.parent_size),
                    self.style.height.height(context.parent_size)
                );

                let mut pos = Vec2::new(
                    match self.style.x {
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
                    match self.style.y {
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
                pos += context.start_pos;


                match self.mode {
                    RenderMode::Inline => {
                        context.start_pos.y += size.y;
                    }
                    _ => ()
                }

                {
                    self.computed.pos = pos;
                    self.computed.size = size;
                    self.computed.color = self.style.color.as_color();
                    self.computed.border_color = self.style.border_color.as_color();
                    self.computed.border = self.style.border;
                    self.computed.corner = self.style.corner.pixelx(size);
                    self.computed.order = context.order;
                }

                let mut context = BuildContext::new_from(context, size, pos, &self.computed as _);

                for element in &mut self.childs {
                    element.build(&mut context);
                    context.order += 1;
                }

                let text = unsafe { (text as *const Text).cast_mut().as_mut().unwrap_unchecked() };

                text.build_text(size, pos, unsafe { &*context.font });
                self.dirty = false;
            },
            UiType::Slider(slider) => {

                let size = Vec2::new(
                    self.style.width.width(context.parent_size) - slider.padding * 2.0,
                    self.style.height.height(context.parent_size) - slider.padding * 2.0
                );

                let mut pos = Vec2::new(
                    match self.style.x {
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
                    match self.style.y {
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

                pos += context.parent_pos;

                match self.mode {
                    RenderMode::Inline => {
                        pos += context.start_pos;
                        context.start_pos += size + slider.padding * 2.0;
                    }
                    _ => ()
                }

                {
                    self.computed.pos = pos;
                    self.computed.size = size;
                    self.computed.color = self.style.color.as_color();
                    self.computed.border_color = self.style.border_color.as_color();
                    self.computed.border = self.style.border;
                    self.computed.corner = self.style.corner.pixelx(size);
                    self.computed.order = context.order;
                }

                let mut context = BuildContext::new_from(context, size - slider.padding * 2.0, pos + slider.padding, &self.computed as _);

                self.childs[0].build(&mut context);
                context.order += 1;
                context.parent_pos.x += slider.padding;
                self.childs[1].build(&mut context);
                self.dirty = false;
            },
            UiType::Image(image) => {
                let size = Vec2::new(
                    self.style.width.width(context.parent_size),
                    self.style.height.height(context.parent_size)
                );

                let mut pos = Vec2::new(
                    match self.style.x {
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
                    match self.style.y {
                        Align::Top(unit) => {
                            unit.pixely(context.parent_size)
                        },
                        Align::Bottom(unit) => {
                            context.parent_size.y - unit.pixely(context.parent_size) - size.y
                        },
                        Align::Center() => {
                            context.parent_size.y * 0.5 - size.y
                        },
                        _ => panic!()
                    },
                );

                pos += context.parent_pos;

                match self.mode {
                    RenderMode::Inline => {
                        pos += context.start_pos;
                        context.start_pos += size;
                    }
                    _ => ()
                }

                {
                    self.computed.pos = pos;
                    self.computed.size = size;
                    self.computed.color = Color::new(f32::from_bits(image.index as u32), 0.0, 0.0, 0.0);
                    self.computed.border_color = self.style.border_color.as_color();
                    self.computed.border = self.style.border;
                    self.computed.corner = self.style.corner.pixelx(size);
                    self.computed.mode = 3;
                    self.computed.order = context.order;
                }
                self.dirty = false;
            }
            _ => {
                let size = Vec2::new(
                    self.style.width.width(context.parent_size),
                    self.style.height.height(context.parent_size)
                );

                let mut pos = Vec2::new(
                    match self.style.x {
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
                    match self.style.y {
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

                pos += context.parent_pos;

                match self.mode {
                    RenderMode::Inline => {
                        pos += context.start_pos;
                        context.start_pos += size;
                    }
                    _ => ()
                }

                {
                    self.computed.pos = pos;
                    self.computed.size = size;
                    self.computed.color = self.style.color.as_color();
                    self.computed.border_color = self.style.border_color.as_color();
                    self.computed.border = self.style.border;
                    self.computed.corner = self.style.corner.pixelx(size);
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
    pub fn get_offset(&self) -> f32 {
        if self.parent.is_null() {
            return 0.0;
        } else {
            let parent = unsafe { &*self.parent };
                let offset;

                if self.computed.order > 0 {
                    let child = &parent.childs[self.computed.order as usize - 1];
                    offset = child.computed.pos.y - parent.computed.pos.y + child.computed.size.y;
                } else {
                    offset = 0.;
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

        self.computed.size.x = style.width.width(parent_size);
        self.computed.size.y = style.height.height(parent_size);
        self.computed.pos.x = match style.x {
            Align::Left(unit) => {
                unit.pixelx(parent_size) + parent_pos.x
            },
            Align::Right(unit) => {
                parent_size.x - unit.pixelx(parent_size) - self.computed.size.x + parent_pos.x
            },
            Align::Center() => {
                (parent_size.x - self.computed.size.x) * 0.5 + parent_pos.x
            },
            _ => panic!()
        };
        self.computed.pos.y = match style.y {
            Align::Top(unit) => {
                unit.pixely(parent_size) + parent_pos.y
            },
            Align::Bottom(unit) => {
                parent_size.y - unit.pixely(parent_size) - self.computed.size.y + parent_pos.y
            },
            Align::Center() => {
                (parent_size.y - self.computed.size.y) * 0.5 + parent_pos.y
            },
            _ => panic!()
        };

        match self.mode {
            RenderMode::Inline => {
                let self_ptr = unsafe { &*(*&self as *const UiElement) };
                self.computed.size.y += self_ptr.get_offset();
            },
            _ => ()
        }

        self.computed.color = style.color.as_color();
        self.computed.border = style.border;
        self.computed.border_color = style.border_color.as_color();
        self.computed.corner = style.corner.pixelx(self.computed.size);

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

        if self.mode == RenderMode::Inline || !self.visible {
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
        Self { style: Default::default(), childs: Default::default(), mode: RenderMode::Absolute, visible: true, dirty: false, computed: RawUiElement::default(), parent: null(), inherit: UiType::Block() }
    }
}