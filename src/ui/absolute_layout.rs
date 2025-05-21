use crate::{graphics::formats::RGBA, primitives::Vec2};

use super::{ui_element::{Element, TypeConst}, Align, BuildContext, ElementType, Padding, RawUiElement, UiUnit, UiElement};

pub struct AbsoluteLayout {
    pub align: Align,
    pub x: UiUnit,
    pub y: UiUnit,
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

impl Element for AbsoluteLayout {
    fn build(&mut self, context: &mut BuildContext) {

        let size = Vec2::new(
            self.width.pixelx(context.parent_size),
            self.height.pixely(context.parent_size)
        );

        let mut pos = self.align.get_pos(context.parent_size, size, Vec2::new(self.x.pixelx(context.parent_size), self.y.pixely(context.parent_size)));

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

impl TypeConst for AbsoluteLayout {
    const ELEMENT_TYPE: ElementType = ElementType::AbsoluteLayout;
}

impl Default for AbsoluteLayout {
    fn default() -> Self {
        Self {
            comp: Default::default(),
            mode: Default::default(),
            childs: Default::default(),
            align: Align::Center,
            x: UiUnit::Px(10.0),
            y: UiUnit::Px(10.0),
            width: UiUnit::Px(100.0),
            height: UiUnit::Px(100.0),
            color: RGBA::DARKGREY,
            border_color: RGBA::GREEN,
            border: [1.0; 4],
            corner: [UiUnit::Relative(0.5); 4],
            padding: Padding::default(),
        }
    }
}