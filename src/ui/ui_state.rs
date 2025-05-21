use std::{mem, ptr::{self, null_mut}};

use ash::vk;

use crate::{graphics::{self, SinlgeTimeCommands, UiInstance, VkBase}, primitives::Vec2};
use super::{raw_ui_element::UiEvent, ui_element::{Element, TypeConst}, ui_pipeline, BuildContext, Font, UiElement};

#[derive()]
pub struct UiState {
    elements: Vec<UiElement>,
    pub selected: *mut UiElement,
    pub pressed: *mut UiElement,
    pub cursor_pos: Vec2,
    pub font: Font,
    pub visible: bool,
    pub dirty: bool,
    id_gen: u32,
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

    pub fn create(visible: bool) -> UiState {
        UiState {
            visible,
            elements: Vec::new(),
            dirty: false,
            id_gen: 1,
            cursor_pos: Vec2::default(),
            selected: null_mut(),
            pressed: null_mut(), 
            font: Font::parse_from_bytes(include_bytes!("../../font/std1.fef")),
            pipeline_layout: vk::PipelineLayout::null(),
            pipeline: vk::Pipeline::null(),
            instance_buffer: graphics::Buffer::null(),
            buffer_size: 0,
            debug: false,
            _debug_instance_buffer: graphics::Buffer::null(),
            _debug_buffer_size: 0,
        }
    }

    pub fn add_element<T: Element + TypeConst + 'static>(&mut self, element: T) {
        let element = UiElement {
            id: self.id_gen,
            typ: T::ELEMENT_TYPE,
            dirty: true,
            visible: true,
            size: Vec2::default(),
            pos: Vec2::default(),
            parent: null_mut(),
            element: Box::new(element),
        };

        self.elements.push(element);
    }

    pub fn init_graphics(&mut self, base: &VkBase, window_size: &winit::dpi::PhysicalSize<u32>, render_pass: vk::RenderPass, descriptor: &vk::DescriptorSetLayout) {
        (self.pipeline_layout, self.pipeline) = ui_pipeline::create_ui_pipeline(base, window_size, render_pass, descriptor);
    }

    pub fn font(&mut self, font: Font) {
        self.font = font;
    }

    pub fn build(&mut self, ui_size: Vec2) {

        self.selected = null_mut();
        self.pressed = null_mut();

        let mut build_context = BuildContext::default(&self.font, ui_size);
        
        for element in &mut self.elements {
            element.build(&mut build_context);
            build_context.order += 1;
        }
    }

    pub fn get_instaces(&mut self, ui_size: Vec2) -> Vec<UiInstance> {
        let mut instances = Vec::new();

        if !self.visible || self.elements.len() == 0 {
            instances.push(UiInstance::default());
            return instances;
        }

        for raw_e in &mut self.elements {
            raw_e.get_instances(&mut instances, ui_size, &self.font);
        }

        self.dirty = false;

        instances
    }

    pub fn get_element_mut(&mut self, root: Vec<usize>) -> Option<&mut UiElement> {
        let mut h = self.elements.get_mut(*root.first()?)?;
        for i in 1..root.len() {
            h = h.element.childs().get_mut(*root.get(i)?)?;
        }

        Some(h)
    }

    pub fn update_cursor(&mut self, _ui_size: Vec2, cursor_pos: Vec2, _event: UiEvent) -> u8 {

        //0 = no event
        //1 = no event break
        //2 = old event
        //3 = new event
        let bol = 0;
        self.cursor_pos = cursor_pos;
        bol
    }

    pub fn update(&mut self, base: &VkBase, new_size: Vec2, command_pool: &vk::CommandPool) {
        self.build(new_size);
        let ui_instances = self.get_instaces(new_size);
        if ui_instances.is_empty() {
            return;
        }
        self.buffer_size = ui_instances.len();
    
        let cmd_buf = SinlgeTimeCommands::begin(base, command_pool);
    
        unsafe {
            let barrier = vk::MemoryBarrier {
                src_access_mask: vk::AccessFlags::TRANSFER_WRITE, // Vorherige Nutzung (Lesen in Shader).
                dst_access_mask: vk::AccessFlags::VERTEX_ATTRIBUTE_READ, // Zielnutzung (Schreiben oder Freigabe).
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
            src_access_mask: vk::AccessFlags::TRANSFER_WRITE,        // Vorherige Nutzung (Lesen in Shader).
            dst_access_mask: vk::AccessFlags::VERTEX_ATTRIBUTE_READ, // Zielnutzung (Schreiben oder Freigabe).
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