use std::ptr::null;

use crate::{graphics::formats::RGBA, primitives::Vec2};

use super::{Align, BuildContext, RawUiElement, Style, UiElement, UiType};

#[derive(Debug, Clone)]
pub struct Slider {
    pub min_value: f32,
    pub max_value: f32,
    pub value: f32,
    pub step: f32,
    pub padding: f32,
}

impl Slider {
    pub fn new(style: Style, min_value: f32, max_value: f32, value: f32, infill_color: RGBA, grip_color: RGBA) -> UiElement {
        let mut childs = Vec::with_capacity(2);
        childs.push(UiElement::inline(Style::infill(5.0, infill_color), Vec::with_capacity(0)));
        childs.push(UiElement::new(Style::toggle(grip_color), Vec::with_capacity(0)));
        UiElement { style,
            visible: true,
            dirty: true,
            parent: null(),
            childs,
            computed: RawUiElement::default(),
            inherit: UiType::Slider(
                Self::slider(min_value, max_value, value),
            )
        }
    }

    pub const fn slider(min_value: f32, max_value: f32, value: f32) -> Self {
        Self { min_value, max_value, value, step: 1.0, padding: 5.0 }
    }

    pub fn build(&self, element: &mut UiElement, context: &mut BuildContext) {
        let size;
        let mut pos;

        match &element.style {
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

                element.computed.color = absolute.color.as_color();
                element.computed.border_color = absolute.border_color.as_color();
                element.computed.border = absolute.border[0];
                element.computed.corner = absolute.corner[0].pixelx(size);
            },
            Style::Inline(inline) => {
                size = Vec2::new(
                    inline.width.width(context.parent_size),
                    inline.height.height(context.parent_size)
                );

                pos = Vec2::new(0.0, 0.0);

                if context.parent_size.x - context.start_pos.x - context.parent_pos.x >= size.x {
                    pos.x += context.start_pos.x;
                    context.start_pos.x += size.x + self.padding * 2.0;
                } else {
                    pos.y += context.start_pos.y;
                    context.start_pos.y += size.y + self.padding * 2.0;
                }

                element.computed.color = inline.color.as_color();
                element.computed.border_color = inline.border_color.as_color();
                element.computed.border = inline.border[0];
                element.computed.corner = inline.corner[0].pixelx(size);
            }
        }

        pos += context.parent_pos;

        {
            element.computed.pos = pos;
            element.computed.size = size;
            element.computed.order = context.order;
        }

        let mut context = BuildContext::new_from(context, size - self.padding * 2.0, pos + self.padding, &element.computed as _);

        element.childs[0].build(&mut context);
        context.order += 1;
        context.parent_pos.x += self.padding;
        element.childs[1].build(&mut context);
        element.dirty = false;
    }
}