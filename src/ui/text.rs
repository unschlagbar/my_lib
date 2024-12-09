
use crate::{graphics::UiInstance, primitives::Vec2, ui::UiSize};

use super::{BuildContext, Style, UiElement, UiType};

#[derive(Debug, Clone)]
pub struct Text {
    pub font: u16,
    pub text: Vec<u8>,
    pub comp_text: Vec<UiInstance>,
    pub wrap: WrapMode,
    pub mode: u8,
}

impl Text {
    pub fn new(style: Style, font: u16, text: Vec<u8>, mode: u8) -> UiElement {
        UiElement::extend(
            style,
            Vec::with_capacity(0),
            UiType::Text(Self::text(font, text, mode)) 
        )
    }

    pub fn text(font: u16, text: Vec<u8>, mode: u8) -> Self {
        Self { font, text, comp_text: Vec::with_capacity(0), wrap: WrapMode::Character,  mode }
    }

    pub fn set_text(&mut self, element: *mut UiElement, text: &[u8]) {
        self.text = text.to_owned();
        let element = unsafe { &mut *element } ;
        element.dirty = true;
    }

    #[inline]
    pub fn build_text(&mut self, style: &Style, size: Vec2, pos: Vec2, context: &mut BuildContext) {
        if self.text.is_empty() {
            println!("no text");
            return;
        }

        if let Style::Inline(style) = style {
            const BSIZE: f32 = 8.0;
            let mut font_size = style.height.height(size);
            let scale_factor = font_size / BSIZE;
            font_size += scale_factor * 2.0;

            let mut current_width: f32 = 0.0;
            let mut max_width = match style.width {
                UiSize::Auto => context.parent_size.x - context.start_pos.x,
                _ => size.x
            };

            let mut lines: Vec<Vec<UiInstance>> = vec![];
            let mut current_line = Vec::new();

            let mut x_progress = pos.x + style.margin[0].pixelx(size);
            let mut y_progress = pos.y + style.margin[1].pixely(size) + scale_factor;

            let mut char_width ;

            println!("sdsf: {}", context.start_pos.x);

            for &c in &self.text {
                if c == b'\n' {
                    lines.push(current_line);
                    current_line = Vec::new();
                    current_width = 0.0;
                    y_progress += font_size;

                    if y_progress - pos.y > context.line_offset {
                        x_progress -= context.start_pos.x;
                        max_width += context.start_pos.x;
                        context.start_pos.x = 0.0;
                    }
                } else {
                    let (uv_x, uv_y, char_w, _) = (unsafe { &*context.font }).get_data(c);
                    char_width = char_w as f32 * scale_factor;

                    //doesnt fit!
                    if current_width + char_width > max_width {
                        lines.push(current_line);
                        current_line = Vec::new();
                        current_width = 0.0;
                        y_progress += font_size;

                        if y_progress - pos.y > context.line_offset {
                            x_progress -= context.start_pos.x;
                            max_width += context.start_pos.x;
                            context.start_pos.x = 0.0;
                        }
                    }

                    let uv_x_w = f32::from_bits(((char_w as u32) << 16) | (uv_x as u32));
                    let uv_y_h = f32::from_bits(((BSIZE as u32) << 16) | (uv_y as u32));

                    current_line.push(UiInstance {
                        color: style.color.as_color(),
                        border_color: style.color.as_color(),
                        border: uv_x_w,
                        x: x_progress + current_width,
                        y: y_progress,
                        width: char_width,
                        height: BSIZE * scale_factor,
                        corner: uv_y_h,
                        mode: 1,
                    });

                    current_width += char_width;
                }
            }

            if !current_line.is_empty() {
                lines.push(current_line);
            }

            let y_offset = match self.mode & 0b10 {
                0 => 0.0,
                2 => size.y * 0.5,
                _ => 0.0,
            };

            let mut final_comp_text = Vec::new();
            for line in &lines {
                let line_width: f32 = line.iter().map(|i| i.width).sum();
                let x_offset = match self.mode & 0b1 {
                    0 => 0.0,
                    1 => (max_width - line_width) * 0.5,
                    _ => 0.0,
                };

                for mut instance in line.clone() {
                    instance.x += x_offset;
                    instance.y += y_offset;
                    final_comp_text.push(instance);
                }
            }

            self.comp_text = final_comp_text;
            context.start_pos.x = current_width;
            context.start_pos.y += font_size * (lines.len() - 1) as f32;
            context.line_offset = font_size;
        } else {
            panic!("Style is not inline");
        }
    }



    pub fn build(&self, element: &mut UiElement, context: &mut BuildContext) {

        let pos;

        match &element.style {
            Style::Absolute(_) => {
                panic!();
            },
            Style::Inline(inline) => {
                pos = Vec2::new(
                    context.start_pos.x,
                    context.start_pos.y
                );

                element.computed.color = inline.color.as_color();
                element.computed.border_color = inline.border_color.as_color();
                element.computed.border = inline.border[0];
            }
        }

        {
            element.computed.pos = pos + context.parent_pos;
            element.computed.order = context.order;
        }

        let text = unsafe { (self as *const Text).cast_mut().as_mut().unwrap_unchecked() };
        text.build_text(&element.style, Vec2::zero(), element.computed.pos, context);
        element.computed.size.x = context.start_pos.x - pos.x;
        element.computed.size.y = context.start_pos.y - pos.y + context.line_offset;
        element.dirty = false;
    }
}

#[derive(Debug, Clone)]
pub enum WrapMode {
    Character,
    Word,
    None,
}