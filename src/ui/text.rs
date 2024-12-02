use std::ptr::null;

use crate::{graphics::{formats::RGBA, UiInstance}, primitives::Vec2};

use super::{Align, BuildContext, Font, RawUiElement, Style, UIUnit, UiElement, UiType};

#[derive(Debug, Clone)]
pub struct Text {
    pub color: RGBA,
    pub size: UIUnit,
    pub margin: (UIUnit, UIUnit, UIUnit, UIUnit),
    pub font: u16,
    pub text: Vec<u8>,
    pub comp_text: Vec<UiInstance>,
    pub mode: u8,
}

impl Text {
    pub fn new(style: Style, color: RGBA, size: UIUnit, font: u16, text: Vec<u8>, mode: u8, margin: (UIUnit, UIUnit, UIUnit, UIUnit)) -> UiElement {
        UiElement { 
            style, 
            visible: true, 
            dirty: false, 
            childs: Vec::with_capacity(0),
            parent: null(),
            computed: RawUiElement::default(), 
            inherit: UiType::Text(Self::text(color, size, font, text, mode, margin)) 
        }
    }

    pub fn text(color: RGBA, size: UIUnit, font: u16, text: Vec<u8>, mode: u8, margin: (UIUnit, UIUnit, UIUnit, UIUnit)) -> Self {
        Self { color, size, margin, font, text, comp_text: Vec::with_capacity(0), mode }
    }

    pub fn set_text(&mut self, element: *mut UiElement, text: &[u8]) {
        self.text = text.to_owned();
        let element = unsafe { &mut *element } ;
        element.dirty = true;
    }

    #[inline]
    pub fn build_text(&mut self, size: Vec2, pos: Vec2, font: &Font) {

        if self.text.len() == 0 {
            println!("no text");
            return;
        }

        const BSIZE: f32 = 8.0;
        let font_size = self.size.pixely(size);
        let scale_factor = font_size / BSIZE;
        let mut a_width: Vec<f32> = vec![0.0];
        let mut w_i = 0;

        let mut o_x = Vec::with_capacity(self.text.len());
        let mut o_y = Vec::with_capacity(self.text.len());
        let mut char_width = Vec::with_capacity(self.text.len());

        for i in &self.text {

            if *i == b'\n' {
                a_width.push(0.0);
                w_i += 1;
            } else {
                let data = font.get_data(*i);
                o_x.push(data.0);
                o_y.push(data.1);
                char_width.push(data.2);
                a_width[w_i] += data.2 as f32;
            }
        }

            a_width[0] *= scale_factor;

        let margin_left = self.margin.0.pixelx(size);
        //let margin_right = text.margin.1.pixelx(parent_width, parent_height);
        let margin_top = self.margin.2.pixely(size);
        let _margin_bottom = self.margin.3.pixely(size);

        let x_start;
        let y_start;

        match self.mode & 0b11 {
            0 => {
                x_start = pos.x + margin_left;
            },
            1 => {
                x_start = (size.x - a_width[0]) * 0.5 + pos.x;
            }
            _ => todo!()
        }

        match self.mode & 0b1100 {
            0 => {
                y_start = pos.y + margin_top + scale_factor * 0.5;
            }
            _ => todo!()
        }

        let mut comp_text = Vec::with_capacity(self.text.len());

        let mut x_progress = x_start;
        for i in 0..o_x.len() {
            let relativ_width = char_width[i] as f32 * scale_factor;

            let bits = ((char_width[i] as u32) << 16) | (o_x[i] as u32);
            let uv_x_w = f32::from_bits(bits);

            let bits = ((BSIZE as u32) << 16) | (o_y[i] as u32);
            let uv_y_h = f32::from_bits(bits);

            comp_text.push(UiInstance { color: self.color.as_color(), border_color: self.color.as_color(), border: uv_x_w, x: x_progress, y: y_start, width: relativ_width, height: BSIZE * scale_factor, corner: uv_y_h, mode: 1});
            x_progress += relativ_width;
        }

        self.comp_text = comp_text;
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
                    context.start_pos.x += size.x;
                } else {
                    pos.y += context.start_pos.y;
                    context.start_pos.y += size.y;
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

        let mut context = BuildContext::new_from(context, size, pos, &element.computed as _);

        for element in &mut element.childs {
            element.build(&mut context);
            context.order += 1;
        }

        let text = unsafe { (self as *const Text).cast_mut().as_mut().unwrap_unchecked() };

        text.build_text(size, pos, unsafe { &*context.font });
        element.dirty = false;
    }
}