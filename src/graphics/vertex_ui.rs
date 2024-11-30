use std::mem::offset_of;

use ash::vk;
use cgmath::Vector2;

use super::formats::{Color, RGB};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct VertexUi {
    pub uv: Vector2<f32>
}

impl VertexUi {
    pub const GET_BINDING_DESCRIPTION: [vk::VertexInputBindingDescription; 2] = [
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<VertexUi>() as _,
            input_rate: vk::VertexInputRate::VERTEX,
        },
        vk::VertexInputBindingDescription {
            binding: 1,
            stride: std::mem::size_of::<UiInstance>() as _,
            input_rate: vk::VertexInputRate::INSTANCE,
        },
    ];

    pub const GET_ATTRIBUTE_DESCRIPTIONS: [vk::VertexInputAttributeDescription; 10] = [
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(VertexUi, uv) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 1,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(UiInstance, color) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 2,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(UiInstance, border_color) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 3,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, border) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 4,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, x) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 5,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, y) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 6,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, width) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 7,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, height) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 8,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, corner) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 1,
            location: 9,
            format: vk::Format::R32_UINT,
            offset: offset_of!(UiInstance, mode) as u32,
        }
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct UiInstance {
    pub color: Color,
    pub border_color: Color,
    pub border: f32,
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub corner: f32,
    pub mode: u32,
}

impl UiInstance {

    pub fn new(color: RGB, x: f32, y: f32, width: f32, height: f32, mode: u32) -> UiInstance {
        UiInstance { color: color.as_color(), border_color: Color::GREY, border: 1.0, x, y, width, height, corner: 3.0, mode }
    }
}
