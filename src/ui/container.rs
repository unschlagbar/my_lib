use crate::{graphics::formats::RGBA, primitives::Vec2};
use super::{
    BuildContext, ElementType,
    Overflow, Padding, 
    RawUiElement, UiUnit,
    UiElement,
    ui_element::{Element, TypeConst},
};

pub struct Container {
    pub margin: [UiUnit; 4],
    pub overflow: Overflow,
    pub width: UiUnit,
    pub height: UiUnit,
    pub color: RGBA,
    pub border_color: RGBA,
    pub border: [f32; 4],
    pub corner: [UiUnit; 4],
    pub padding: Padding,
    pub comp: RawUiElement,
    pub mode: u8,
    pub childs: Vec<UiElement>,
}

impl Element for Container {
    fn build(&mut self, context: &mut BuildContext) {
        let mut size;
        let mut pos;

        let space = Vec2::new(
            context.parent_size.x - self.padding.x(context.parent_size),
            context.parent_size.y -  self.padding.y(context.parent_size)
        );

        size = Vec2::new(
            self.width.pixelx(space),
            self.height.pixely(space)
        );

        let mut outer_size = size + Vec2::new(
            self.margin[2].pixelx(context.parent_size),
            self.margin[3].pixely(context.parent_size),
        );

        pos = Vec2::new(
            self.margin[0].pixelx(context.parent_size),
            self.margin[1].pixely(context.parent_size),
        );

        context.fits_in_line(&mut pos, &mut outer_size);

        let comp = &mut self.comp;

        comp.color = self.color.as_color();
        comp.border_color = self.border_color.as_color();
        comp.border = self.border[0];
        comp.corner = self.corner[0].pixelx(size);

        pos += context.parent_pos;
                
        comp.size = size;
        comp.pos = pos;

        let mut context = BuildContext::new_from(context, size, pos, &comp);

        for element in self.childs.iter_mut() {
            element.build(&mut context);
            context.order += 1;
        }


        if self.width == UiUnit::Auto && context.start_pos.x != 0.0 {
            size.x = context.start_pos.x
        }
        if self.height == UiUnit::Auto && context.start_pos.y != 0.0 {
            comp.size.y = context.start_pos.y + context.line_offset
        }
    }

    fn instance(&self) -> crate::graphics::UiInstance {
        self.comp.to_instance()
    }

    fn childs(&mut self) -> &mut[UiElement] {
        &mut self.childs
    }

    fn add_child(&mut self, child: UiElement) {
        self.childs.push(child);
    }

    fn comp(&mut self) -> &mut RawUiElement {
        &mut self.comp
    }
}

impl TypeConst for Container {
    const ELEMENT_TYPE: ElementType = ElementType::Block;
}

impl Default for Container {
    fn default() -> Self {
        Self {
            comp: Default::default(),
            mode: Default::default(),
            childs: Default::default(),
            margin: [UiUnit::Zero; 4],
            overflow: Overflow::hidden(),
            width: UiUnit::Px(100.0),
            height: UiUnit::Px(100.0),
            color: RGBA::DARKGREY,
            border_color: RGBA::GREEN,
            border: [1.0; 4],
            corner: [UiUnit::Px(5.0); 4],
            padding: Padding::default(),
        }
    }
}