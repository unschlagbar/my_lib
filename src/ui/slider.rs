
use crate::{graphics::formats::RGBA, primitives::Vec2};
use std::fmt::Debug;
use super::{callback::ErasedFnPointer, dragbox::DragEvent, style::Position, BuildContext, DragBox, Style, UiElement, UiType};

#[derive(Clone)]
pub struct Slider {
    pub min_value: f32,
    pub max_value: f32,
    pub value: f32,
    pub step: f32,
    pub padding: f32,
    pub on_slide: ErasedFnPointer
}

impl Slider {
    #[inline(always)]
    pub fn new(style: Style, min_value: f32, max_value: f32, value: f32, infill_color: RGBA, grip_color: RGBA) -> UiElement {

        let mut dragbox = DragBox {
            axis: 1,
            ..Default::default()
        };

        let mut slider = UiElement::extend(
            style,
            Vec::with_capacity(2),
            UiType::Slider(
               Self::slider(min_value, max_value, value),
            )
        );

        dragbox.on_drag(|event| {
            Self::on_drag(event);
        });

        slider.add_child(UiElement::inline(Style::infill(3.0, infill_color), Vec::with_capacity(0)));
        slider.add_child(UiElement { style: Style::toggle(grip_color), inherit: UiType::DragBox(dragbox), ..Default::default()});

        slider
    }

    pub const fn slider(min_value: f32, max_value: f32, value: f32) -> Self {
        Self { min_value, max_value, value, step: 1.0, padding: 5.0, on_slide: ErasedFnPointer::null() }
    }

    pub fn build(&self, element: &mut UiElement, context: &mut BuildContext) {
        let mut size;
        let mut pos;

        match &element.style.position {
            Position::Absolute(absolute) => {
                size = Vec2::new(
                    element.style.width.width(context.parent_size),
                    element.style.height.height(context.parent_size)
                );

                pos = absolute.align.get_pos(context.parent_size, size, Vec2::new(absolute.x.pixelx(context.parent_size), absolute.y.pixely(context.parent_size)));

                element.computed.color = element.style.color.as_color();
                element.computed.border_color = element.style.border_color.as_color();
                element.computed.border = element.style.border[0];
                element.computed.corner = element.style.corner[0].pixelx(size);
            },
            Position::Inline(inline) => {
                let space = Vec2::new(
                    context.parent_size.x - inline.margin[0].pixelx(context.parent_size) - inline.margin[2].pixelx(context.parent_size),
                    context.parent_size.y -  inline.margin[1].pixely(context.parent_size) - inline.margin[3].pixely(context.parent_size)
                );

                size = Vec2::new(
                    element.style.width.width(space),
                    element.style.height.height(space)
                );

                pos = Vec2::new(
                    inline.margin[0].pixelx(context.parent_size),
                    inline.margin[1].pixely(context.parent_size),
                );

                context.fits_in_line(inline, &mut pos, &mut size);

                element.computed.color = element.style.color.as_color();
                element.computed.border_color = element.style.border_color.as_color();
                element.computed.border = element.style.border[0];
                element.computed.corner = element.style.corner[0].pixelx(size);
            }
        }

        let padding = element.style.padding;

        pos += context.parent_pos;

        {
            element.computed.pos = pos;
            element.computed.size = size;
            element.computed.order = context.order;
        }

        let mut context = BuildContext::new_from(context, size - Vec2::new(padding.x(), padding.y()), pos + padding.start(), &element.computed as _);

        element.childs[0].build(&mut context);
        element.childs[0].parent = element as *mut UiElement;
        context.order += 1;
        context.parent_pos.x += self.padding;
        element.childs[1].build(&mut context);
        element.childs[1].parent = element as *mut UiElement;
        element.dirty = false;
    }

    pub fn on_drag(event: &mut DragEvent) {
        let slider = unsafe { &mut *event.element.parent };
        let assumed_pos = event.element.computed.pos.x + event.move_vec.x;
        let slider_space = slider.computed.size.x - event.element.computed.size.x;
        let relative_toggle_pos = event.element.computed.pos.x - slider.computed.pos.x;

        if assumed_pos < slider.computed.pos.x {
            event.move_vec.x = slider.computed.pos.x - event.element.computed.pos.x;
        } else if assumed_pos + event.element.computed.size.x > slider.computed.pos.x + slider.computed.size.x {
            event.move_vec.x = slider_space - relative_toggle_pos;
        }
            
        let t = relative_toggle_pos / slider_space;

        let slider_component = match &mut slider.inherit {
            UiType::Slider(slider) => slider,
            _ => unreachable!()
        };

        slider_component.value = slider_component.min_value + t * (slider_component.max_value - slider_component.min_value);
            
        let parent = unsafe { &mut *slider.parent };
        let text = &mut parent.childs[1];
        let text_copy = text as *mut UiElement;

        match &mut text.inherit {
            UiType::Text(text) => {
                text.set_text(text_copy, &format!("Value: {}", slider_component.value as u32))
            },
            _ => unreachable!()
        };
    }
}

impl Debug for Slider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UiButton").finish()
    }
}