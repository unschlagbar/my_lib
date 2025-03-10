use std::ptr::null_mut;

use crate::{graphics::{formats::Color, UiInstance}, primitives::Vec2};

use super::{style::Position, ui_state::UiIndex, BuildContext, Font, Interaction, RawUiElement, Style, UiEvent, UiSize, UiState, UiType};

#[derive(Debug, Clone)]
pub struct UiElement {
    pub style: Style,
    pub visible: bool,
    pub dirty: bool,
    pub parent: *mut UiElement,
    pub childs: Vec<UiElement>,
    pub computed: RawUiElement,
    pub inherit: UiType
}

impl UiElement {

    pub fn extend(style: Style, childs: Vec<UiElement>, inherit: UiType) -> Self {
        Self { 
            style,
            visible: true,
            dirty: true,
            parent: null_mut(),
            childs,
            computed: RawUiElement::default(),
            inherit
        }
    }

    pub fn new(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null_mut() }
    }

    pub fn inline(style: Style, childs: Vec<UiElement>) -> Self {
        Self { style, childs, visible: true, dirty: true, inherit: UiType::Block(), computed: RawUiElement::default(), parent: null_mut() }
    }

    #[inline(always)]
    pub fn build(&mut self, context: &mut BuildContext) {

        let self_copy = unsafe { &mut *(self as *mut _) };
        let self_ptr: *mut UiElement = self;

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

                let mut size;
                let mut pos;

                match &self.style.position {
                    Position::Absolute(absolute) => {
                        size = Vec2::new(
                            self.style.width.width(context.parent_size),
                            self.style.height.height(context.parent_size)
                        );

                        pos = absolute.align.get_pos(context.parent_size, size, Vec2::new(absolute.x.pixelx(context.parent_size), absolute.y.pixely(context.parent_size)));
                    },
                    Position::Inline(inline) => {

                        let space = Vec2::new(
                            context.parent_size.x - inline.margin[0].pixelx(context.parent_size) - inline.margin[2].pixelx(context.parent_size),
                            context.parent_size.y -  inline.margin[1].pixely(context.parent_size) - inline.margin[3].pixely(context.parent_size)
                        );

                        size = Vec2::new(
                            self.style.width.width(space),
                            self.style.height.height(space)
                        );

                        pos = Vec2::new(
                            inline.margin[0].pixelx(context.parent_size),
                            inline.margin[1].pixely(context.parent_size),
                        );

                        context.fits_in_line(inline, &mut pos, &mut size);
                    }
                }

                let style = self.style;

                self.computed.color = style.color.as_color();
                self.computed.border_color = style.border_color.as_color();
                self.computed.border = style.border[0];
                self.computed.corner = style.corner[0].pixelx(size);

                pos += context.parent_pos;
                
                self.computed.order = context.order;
                self.computed.size = size;
                self.computed.pos = pos;

                let mut context = BuildContext::new_from(context, size, pos, &self.computed as _);

                for element in &mut self.childs {
                    element.build(&mut context);
                    element.parent = self_ptr;
                    context.order += 1;
                }

                self.dirty = false;

                if let Position::Inline(_) = &self.style.position {
                    if UiSize::Auto == style.width && context.start_pos.x != 0.0 {
                        self.computed.size.x = context.start_pos.x
                    }
                    if UiSize::Auto == style.height && context.start_pos.y != 0.0 {
                        self.computed.size.y = context.start_pos.y + context.line_offset
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn get_instances(&mut self, instances: &mut Vec<UiInstance>, ui_size: Vec2, font: &Font) {

        if self.dirty {
            if self.parent.is_null() {
                self.rebuild(ui_size, Vec2::default(), font);
            } else {
                let parent = unsafe { &*self.parent };
                self.rebuild(parent.computed.size, parent.computed.pos, font);
            }
        }

        if !self.visible {
            return;
        }
        
        if let UiType::Text(text) = &self.inherit {
            instances.extend_from_slice(text.comp_text.as_slice());
        } else {
            instances.push(self.computed.to_instance());
        }

        for child in &mut self.childs {
            child.get_instances(instances, ui_size, font);
        }
    }

    #[inline(always)]
    pub fn get_outline(&mut self, instances: &mut Vec<UiInstance>, ui_size: Vec2) {

        if let UiType::Text(_text) = &self.inherit {
            println!("todo");
        } else {
            instances.push(UiInstance { color: Color::new(0.1, 0.1, 0.8, 0.5), border_color: Color::PURPLE, border: 1.0, x: self.computed.pos.x, y: self.computed.pos.y, width: self.computed.size.x, height: self.computed.size.y, corner: 0.0, mode: 0 });
        }

        for child in &mut self.childs {
            child.get_outline(instances, ui_size);
        }
    }

    #[inline(always)]
    pub fn get_offset(&self) -> Vec2 {
        let offset;
        if !self.parent.is_null() {
            let parent = unsafe { &*self.parent };
            
            if self.computed.order > 0 {
                let child = &parent.childs[self.computed.order as usize - 1];
                offset = child.computed.pos - parent.computed.pos + child.computed.size;
            } else {
                offset = Vec2::default();
            }
        } else {
            offset = Vec2::default();
        }
        offset
    }

    #[inline(always)]
    pub fn move_computed(&mut self, amount: Vec2) {
        for child in &mut self.childs {
            child.move_computed(amount);
        }
        self.computed.pos += amount;

        if let UiType::Text(text) = &mut self.inherit {
            for raw in &mut text.comp_text {
                raw.x += amount.x;
                raw.y += amount.y;
            }
        }
    }

    #[inline(always)]
    pub fn move_computed_absolute(&mut self, pos: Vec2) {
        for child in &mut self.childs {
            child.move_computed_absolute(pos);
        }
        self.computed.pos = pos;
    }

    #[inline(always)]
    pub fn rebuild(&mut self, parent_size: Vec2, parent_pos: Vec2, font: &Font) {

        let style: &Style = match &self.inherit {
            UiType::Button(button) => {
                if button.interaction == Interaction::Pressed {
                    &button.press_style
                } else if button.interaction == Interaction::Hover {
                    &button.hover_style
                } else {
                    &self.style
                }
            },
            UiType::CheckBox(checkbox) => {
                if checkbox.enabled {
                    &checkbox.press_style
                } else if checkbox.selected {
                    &checkbox.hover_style
                } else {
                    &self.style
                }
            }
            _ => &self.style,
        };

        let mut context = BuildContext::default(font, parent_size);

        match style.position {
            Position::Absolute(absolute) => {
                self.computed.size = Vec2::new(
                    style.width.width(parent_size),
                    style.height.height(parent_size)
                );

                self.computed.pos = absolute.align.get_pos(parent_size, self.computed.size, Vec2::new(absolute.x.pixelx(parent_size), absolute.y.pixely(parent_size)));
            },
            Position::Inline(inline) => {

                let space = Vec2::new(
                    parent_size.x - inline.margin[0].pixelx(parent_size) - inline.margin[2].pixelx(parent_size),
                    parent_size.y -  inline.margin[1].pixely(parent_size) - inline.margin[3].pixely(parent_size)
                );
                
                let old_pos = self.computed.pos;
                let old_size = self.computed.size;

                self.computed.size = Vec2::new(
                    style.width.width(space),
                    style.height.height(space)
                );

                self.computed.pos = parent_pos + Vec2::new(
                    inline.margin[0].pixelx(parent_size),
                    inline.margin[1].pixely(parent_size),
                );

                let original_start_pos = self.get_offset();

                context.parent_pos = parent_pos;
                context.line_offset = self.computed.size.y;
                context.start_pos = original_start_pos;

                context.fits_in_line(&inline, &mut self.computed.pos, &mut self.computed.size);

                self.computed.pos = old_pos;

                if self.computed.pos != old_pos && self.computed.size != old_size && false {

                    let neightbours = &mut (unsafe { &mut*self.parent }).childs;

                    for i in (self.computed.order as usize) + 1..neightbours.len() {
                        let neightbour: &mut UiElement = unsafe { neightbours.get_unchecked_mut(i) };

                        if context.parent_size.x - context.start_pos.x >= neightbour.computed.size.x {
                            neightbour.move_computed_absolute(original_start_pos);

                            context.line_offset = context.line_offset.max(neightbour.computed.size.y + inline.margin[1].pixely(context.parent_size) + inline.margin[3].pixely(context.parent_size));
                            context.start_pos.x += neightbour.computed.size.x + inline.margin[0].pixelx(context.parent_size) + inline.margin[2].pixelx(context.parent_size);

                        } else {
                            neightbour.move_computed(Vec2::new(0.0, context.line_offset));
                            neightbour.computed.pos.y += context.start_pos.y;
                            context.line_offset = neightbour.computed.size.y + inline.margin[1].pixely(context.parent_size) + inline.margin[3].pixely(context.parent_size);
                            context.start_pos.x = neightbour.computed.size.x + inline.margin[0].pixelx(context.parent_size) + inline.margin[2].pixelx(context.parent_size);
                        }
                    }
                }

            }
        };

        self.computed.color = style.color.as_color();
        self.computed.border_color = style.border_color.as_color();
        self.computed.border = style.border[0];
        self.computed.corner = style.corner[0].pixelx(self.computed.size);

        if let UiType::Text(text) = &mut self.inherit {
            text.build_text(&self.style, Vec2::zero(), self.computed.pos, &mut context);
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
        //1 = no event break
        //2 = old event
        //3 = new event

        if !self.visible {
            return 0;
        }

        let computed = &self.computed;

        if computed.pos < cursor_pos {

            if computed.pos.x + computed.size.x > cursor_pos.x && computed.pos.y + computed.size.y > cursor_pos.y {

                for child in self.childs.iter_mut().rev() {
                    let result = child.update_cursor(ui, computed.size, computed.pos, cursor_pos, ui_event);
                    if result > 0 { return result };
                }

                let self_ptr: *mut UiElement = self;

                match ui_event {
                    UiEvent::Press => {
                        match &mut self.inherit {
                            UiType::Button(button) => {
                                button.interaction = Interaction::Pressed;
                                button.before_press.call(ui, unsafe { &mut *self_ptr });
                                self.dirty = true;
                                ui.pressed = UiIndex::new(self, usize::MAX);
                            },
                            UiType::DragBox(dragbox) => {
                                dragbox.interaction = Interaction::Pressed;
                                ui.pressed = UiIndex::new(self, usize::MAX);
                                return 1;
                            },
                            UiType::CheckBox(checkbox) => {
                                checkbox.pressed = true;
                                ui.pressed = UiIndex::new(self, usize::MAX);
                                self.dirty = true;
                            },
                            _ => return 1
                        };
                    },
                    UiEvent::Release => {
                        match &self.inherit {
                            UiType::Button(_) => (),
                            UiType::CheckBox(_) => (),
                            _ => return 1
                        };
                    },
                    UiEvent::Move => {
                        match &mut self.inherit {
                            UiType::Button(button) => {
                                if button.interaction != Interaction::Hover {

                                    button.interaction = Interaction::Hover;
                                    self.dirty = true;

                                    if !ui.selected.is_null() {
                                        let raw_ref = ui.selected.get_by_ptr();
                                        match unsafe { &mut raw_ref.inherit } {
                                            UiType::Button(b) => {
                                                b.interaction = Interaction::None;
                                                raw_ref.dirty = true;
                                            },
                                            UiType::CheckBox(b) => {
                                                b.selected = false;
                                                raw_ref.dirty = true;
                                            },
                                            _ => ()
                                        }
                                    }

                                    ui.selected = UiIndex::new(self, usize::MAX);
                                } else {
                                    return 2;
                                }
                            },
                            UiType::CheckBox(checkbox) => {
                                if !checkbox.selected {

                                    checkbox.selected = true;
                                    self.dirty = true;

                                    if !ui.selected.is_null() {
                                        let raw_ref = ui.selected.get_by_ptr();
                                        match unsafe { &mut raw_ref.inherit } {
                                            UiType::Button(b) => {
                                                b.interaction = Interaction::None;
                                                raw_ref.dirty = true;
                                            },
                                            UiType::CheckBox(b) => {
                                                b.selected = false;
                                                raw_ref.dirty = true;
                                            },
                                            _ => ()
                                        }
                                    }

                                    ui.selected = UiIndex::new(self, usize::MAX);
                                } else {
                                    return 2;
                                }
                            },
                            _ => return 1
                        }
                    },
                };
                return 3;
            }
        }
        0
    }

    #[inline]
    pub fn set_dirty(&self) {
        unsafe { (self as *const UiElement as *mut UiElement).as_mut().unwrap_unchecked().dirty = true };
    }

    #[inline]
    pub fn add_child(&mut self, child: Self) {
        self.childs.push(child);
    }
}

impl Default for UiElement {
    fn default() -> Self {
        Self { style: Default::default(), childs: Default::default(), visible: true, dirty: false, computed: RawUiElement::default(), parent: null_mut(), inherit: UiType::Block() }
    }
}