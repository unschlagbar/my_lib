use std::{mem, ptr::{self, null_mut}};

use ash::vk;

use crate::{graphics::{self, SinlgeTimeCommands, UiInstance, VkBase}, primitives::Vec2};
use super::{dragbox::DragEvent, raw_ui_element::UiEvent, style::Position, ui_pipeline, BuildContext, Font, Interaction, Style, UiElement, UiType};

#[derive(Debug)]
pub struct UiState {
    elements: Vec<UiElement>,
    pub selected: UiIndex,
    pub pressed: UiIndex,
    pub cursor_pos: Vec2,
    pub font: Font,
    pub visible: bool,
    pub dirty: bool,
    pipeline_layout: vk::PipelineLayout,
    pipeline: vk::Pipeline,
    instance_buffer: graphics::Buffer,
    buffer_size: usize,
    pub debug: bool,
    _debug_instance_buffer: graphics::Buffer,
    _debug_buffer_size: usize,
    //text_pipeline: vk::Pipeline,
    //text_pipeline_layout: vk::PipelineLayout,
}

impl UiState {

    pub fn create(elements: Vec<UiElement>, _styles: Vec<Style>, visible: bool) -> UiState {
        UiState {
            visible,
            elements,
            dirty: false,
            cursor_pos: Vec2::default(),
            selected: UiIndex::null(),
            pressed: UiIndex::null(), 
            font: Font::parse_from_bytes(include_bytes!("C:/Dev/vudeljump/font/std1.fef")),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            instance_buffer: graphics::Buffer::null(),
            buffer_size: 0,
            debug: false,
            _debug_instance_buffer: graphics::Buffer::null(),
            _debug_buffer_size: 0,
        }
    }

    pub fn init_graphics(&mut self, base: &VkBase, window_size: &winit::dpi::PhysicalSize<u32>, render_pass: vk::RenderPass, descriptor: &vk::DescriptorSetLayout) {
        (self.pipeline_layout, self.pipeline) = ui_pipeline::create_ui_pipeline(base, window_size, render_pass, descriptor);
    }

    pub fn font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn build(&mut self, ui_size: Vec2) {

        self.selected = UiIndex::null();
        self.pressed = UiIndex::null();

        let mut build_context = BuildContext::default(&self.font, ui_size);
        
        for element in &mut self.elements {
            element.build(&mut build_context);
            build_context.order += 1;
        }
    }

    pub fn get_instaces(&mut self, ui_size: Vec2) -> Vec<UiInstance> {
        let mut instances = Vec::new();

        if !self.visible || self.elements.len() == 0 {
            instances.push(UiElement::default().computed.to_instance());
            return instances;
        }

        for raw_e in &mut self.elements {
            raw_e.get_instances(&mut instances, ui_size, &self.font);
        }

        self.dirty = false;

        instances
    }

    pub fn get_debug_outlines(&mut self, ui_size: Vec2) -> Vec<UiInstance> {
        let mut instances = Vec::new();

        if !self.visible || self.elements.len() == 0 {
            instances.push(UiElement::default().computed.to_instance());
            return instances;
        }

        for raw_e in &mut self.elements {
            raw_e.get_instances(&mut instances, ui_size, &self.font);
        }

        self.dirty = false;

        instances
    }

    pub unsafe fn get_element_mut(&mut self, root: Vec<usize>) -> Option<&mut UiElement> {
        let mut h = &mut self.elements[*root.first()?];
        for i in 1..root.len() {
            h = &mut h.childs[*root.get(i)?];
        }

        Some(h)
    }

    pub fn get_element(&self, root: Vec<usize>) -> Option<&UiElement> {
        let mut h = &self.elements[*root.first()?];
        for i in 1..root.len() {
            h = &h.childs[*root.get(i)?];
        }

        Some( &h )
    }

    pub fn update_cursor(&mut self, ui_size: Vec2, cursor_pos: Vec2, event: UiEvent) -> u8 {

        //0 = no event
        //1 = no event break
        //2 = old event
        //3 = new event
        let mut bol = 0;

        let ui = unsafe { &mut *(self as *const UiState).cast_mut() };

        for i in self.elements.iter_mut().rev() {
            let result = i.update_cursor(ui, ui_size, Vec2::default(), cursor_pos, event);
            if result > 0 {
                bol = result;
                break;
            }
        }

        //Not old event
        if bol != 2 {
            if !self.selected.is_null() && bol < 2 {
                let selected_ptr = self.selected.ptr;
                self.selected = UiIndex::null();
                let selected = unsafe { &mut *selected_ptr };
                match &mut selected.inherit {
                    UiType::Button(button) => {
                        if button.interaction == Interaction::Hover {
                            button.interaction = Interaction::None;
                            selected.dirty = true;
                            self.dirty = true;
                            return 3;
                        }
                    },
                    UiType::CheckBox(checkbox) => {
                        if checkbox.selected {
                            checkbox.selected = false;
                            selected.dirty = true;
                            self.dirty = true;
                            return 3;
                        }
                    }
                    _ => todo!()
                }
            }
            if !self.pressed.is_null() {
                let element = self.pressed.get_by_ptr();
                let is_in = element.is_in(cursor_pos);
                match event {
                    UiEvent::Move => {
                        match &element.inherit {
                            UiType::DragBox(drag) => {
                                match element.style.position {
                                    Position::Inline(_) if !element.parent.is_null() => {
                                        let parent = unsafe { &mut *element.parent };
                                        let mut move_vec = match drag.axis {
                                            1 => Vec2::new(1.0, 0.0),
                                            2 => Vec2::new(0.0, 1.0),
                                            3 => Vec2::one(),
                                            0 => Vec2::zero(),
                                            _ => unreachable!()

                                        } * (cursor_pos - self.cursor_pos);

                                        if !drag.on_drag.is_null() {
                                            let fn_ptr = drag.on_drag;
                                            let mut event = DragEvent { move_vec, element };
                                            
                                            fn_ptr.call_vars(&mut event);
                                            move_vec = event.move_vec;
                                        }

                                        parent.move_computed(move_vec);
                                    },
                                    _ => {
                                        let mut move_vec = match drag.axis {
                                            1 => Vec2::new(1.0, 0.0),
                                            2 => Vec2::new(0.0, 1.0),
                                            3 => Vec2::one(),
                                            0 => Vec2::zero(),
                                            _ => unreachable!()

                                        } * (cursor_pos - self.cursor_pos);

                                        if !drag.on_drag.is_null() {
                                            let fn_ptr = drag.on_drag;
                                            let mut event = DragEvent { move_vec, element };
                                            
                                            fn_ptr.call_vars(&mut event);
                                            move_vec = event.move_vec;
                                        }

                                        element.computed.pos += move_vec;
                                    }
                                }
                                bol = 3;
                            }
                            _ => ()
                        }
                    },
                    UiEvent::Release => {
                        match &mut element.inherit {
                            UiType::Button(button) => {
                                button.interaction = Interaction::None;
                                element.dirty = true;
                                bol = 3;
                                let button2 = button as *const _ as *mut _;
                                if is_in {
                                    #[allow(invalid_reference_casting)]
                                    button.on_press.call(ui, unsafe { &mut *button2 });
                                }
                                self.pressed = UiIndex::null();
                            },
                            UiType::CheckBox(checkbox) => {
                                checkbox.pressed = false;
                                element.dirty = true;
                                bol = 3;
                                let checkbox2 = checkbox as *const _ as *mut _;
                                if is_in {
                                    if checkbox.enabled {
                                        #[allow(invalid_reference_casting)]
                                        checkbox.on_disable.call(ui, unsafe { &mut *checkbox2 });
                                    } else {
                                        #[allow(invalid_reference_casting)]
                                        checkbox.on_enable.call(ui, unsafe { &mut *checkbox2 });
                                    }
                                    checkbox.enabled = !checkbox.enabled;
                                }
                                self.pressed = UiIndex::null();
                            },
                            UiType::DragBox(dragbox) => {
                                dragbox.interaction = Interaction::None;
                                self.pressed = UiIndex::null();
                            },
                            _ => ()
                        }
                    },
                    _ => (),
                }
            }
        }
        //New event
        if bol == 3 { self.dirty = true }
        self.cursor_pos = cursor_pos;
        bol
    }

    pub fn update(&mut self, base: &VkBase, new_size: Vec2, command_pool: &vk::CommandPool) {
        self.build(new_size);
        let ui_instances = self.get_instaces(new_size);
        self.buffer_size = ui_instances.len();
    
        let cmd_buf = SinlgeTimeCommands::begin(base, command_pool);
    
        unsafe {
            let barrier = vk::MemoryBarrier {
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE, // Vorherige Nutzung (Lesen in Shader).
                dst_access_mask: vk::AccessFlags::VERTEX_ATTRIBUTE_READ,       // Zielnutzung (Schreiben oder Freigabe).
                ..Default::default()
            };
        
            base.device.cmd_pipeline_barrier(
                cmd_buf,
                vk::PipelineStageFlags::TRANSFER,
                vk::PipelineStageFlags::VERTEX_INPUT,
                vk::DependencyFlags::empty(),
                &[barrier],
                &[],
                &[],
            );
            
                // Lösche den Puffer erst nach der Barrier.
            SinlgeTimeCommands::end(base, command_pool, cmd_buf);
            self.instance_buffer.destroy(&base.device);
        };
    
        self.instance_buffer = graphics::Buffer::device_local(&base, &command_pool, size_of::<UiInstance>() as u64, ui_instances.len() as u64, ui_instances.as_ptr() as _, vk::BufferUsageFlags::VERTEX_BUFFER);
    }

    pub fn upload(&mut self, base: &VkBase, size: Vec2, command_pool: &vk::CommandPool) {
        let ui_instances = self.get_instaces(size);

        let buffer_size = size_of::<UiInstance>() as u64 * ui_instances.len() as u64;

        let staging_buffer= graphics::Buffer::create(&base, buffer_size, vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        let mapped_memory = staging_buffer.map_memory(&base.device, buffer_size);
        unsafe {
            ptr::copy_nonoverlapping(ui_instances.as_ptr(), mapped_memory as _, ui_instances.len());
        };
        staging_buffer.unmap_memory(&base.device);

        
        if buffer_size != self.buffer_size as u64 * mem::size_of::<UiInstance>() as u64 {
            unsafe { base.device.queue_wait_idle(base.queue).unwrap() };
            self.instance_buffer.destroy(&base.device);
            self.instance_buffer = graphics::Buffer::create(&base, buffer_size, vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::TRANSFER_DST, vk::MemoryPropertyFlags::DEVICE_LOCAL);
        }
        
        let barrier = vk::MemoryBarrier {
            src_access_mask: vk::AccessFlags::TRANSFER_WRITE, // Vorherige Nutzung (Lesen in Shader).
            dst_access_mask: vk::AccessFlags::VERTEX_ATTRIBUTE_READ,       // Zielnutzung (Schreiben oder Freigabe).
            ..Default::default()
        };
        
        let cmd_buf = SinlgeTimeCommands::begin(&base, command_pool);
    
        unsafe { base.device.cmd_pipeline_barrier(
            cmd_buf,
            vk::PipelineStageFlags::TRANSFER,
            vk::PipelineStageFlags::VERTEX_INPUT,
            vk::DependencyFlags::empty(),
            &[barrier],
            &[],
            &[],
        ) };

        staging_buffer.copy(&self.instance_buffer, &base, buffer_size, cmd_buf);


        SinlgeTimeCommands::end(base, command_pool, cmd_buf);
        staging_buffer.destroy(&base.device);

        self.buffer_size = ui_instances.len();
    }

    pub fn draw(&self, device: &ash::Device, cmd: vk::CommandBuffer, descriptor_set: &vk::DescriptorSet) {
        unsafe {
            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline);
            device.cmd_bind_vertex_buffers(cmd, 0, &[self.instance_buffer.inner], &[0]);
            device.cmd_bind_descriptor_sets(cmd, vk::PipelineBindPoint::GRAPHICS, self.pipeline_layout, 0, &[*descriptor_set], &[]);
            device.cmd_draw(cmd, 4, self.buffer_size as _, 0, 0);
        }
    }

    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.free_memory(self.instance_buffer.mem, None);
            device.destroy_buffer(self.instance_buffer.inner, None);
        }
    }

}

#[derive(Debug)]
pub struct UiIndex {
    pub ptr: *mut UiElement,
    pub index: usize,
}

impl UiIndex {
    pub fn new(ptr: &mut UiElement, index: usize) -> UiIndex {
        UiIndex { ptr: ptr as *mut UiElement, index }
    }

    pub fn null() -> UiIndex {
        UiIndex { ptr: null_mut(), index: usize::MAX }
    }

    pub fn get_by_ptr(&self) -> &mut UiElement {
        unsafe { &mut *self.ptr }
    }

    pub fn is_null(&self) -> bool {
        self.ptr.is_null() && self.index == usize::MAX
    }
}