
use crate::{graphics::formats::Color, primitives::Vec2};

use super::{Align, BuildContext, Style, UiElement};



#[derive(Debug, Clone)]
pub struct UiImage {
    pub index: u8,
}

impl UiImage {
    pub fn new(style: Style, index: u8) -> UiElement {
        UiElement::extend(
            style,
            Vec::with_capacity(0),
            super::UiType::Image(Self { index })
        )
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
                    context.start_pos.x += size.x;
                } else {
                    pos.y += context.start_pos.y;
                    context.start_pos.y += size.y;
                }

                element.computed.border_color = inline.border_color.as_color();
                element.computed.border = inline.border[0];
                element.computed.corner = inline.corner[0].pixelx(size);
            }
        }

        pos += context.parent_pos;

        {
            element.computed.pos = pos;
            element.computed.size = size;
            element.computed.color = Color::new(f32::from_bits(self.index as u32), 0.0, 0.0, 0.0);
            element.computed.mode = 3;
            element.computed.order = context.order;
        }
        element.dirty = false;
    }
}