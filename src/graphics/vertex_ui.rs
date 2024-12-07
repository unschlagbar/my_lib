use std::mem::offset_of;

use ash::vk;

use super::formats::{Color, RGB};

#[derive(Debug, Clone, PartialEq)]
#[repr(C)]
pub struct VertexUi;

impl VertexUi {
    pub const GET_BINDING_DESCRIPTION: [vk::VertexInputBindingDescription; 1] = [
        vk::VertexInputBindingDescription {
            binding: 0,
            stride: std::mem::size_of::<UiInstance>() as _,
            input_rate: vk::VertexInputRate::INSTANCE,
        },
    ];

    pub const GET_ATTRIBUTE_DESCRIPTIONS: [vk::VertexInputAttributeDescription; 9] = [
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 0,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(UiInstance, color) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 1,
            format: vk::Format::R32G32B32A32_SFLOAT,
            offset: offset_of!(UiInstance, border_color) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 2,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, border) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 3,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, x) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 4,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, y) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 5,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, width) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 6,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, height) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 7,
            format: vk::Format::R32_SFLOAT,
            offset: offset_of!(UiInstance, corner) as u32,
        },
        vk::VertexInputAttributeDescription {
            binding: 0,
            location: 8,
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
