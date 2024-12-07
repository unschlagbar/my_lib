use std::ffi::c_void;

use ash::vk::{self, MemoryAllocateFlags};

use super::{SinlgeTimeCommands, VkBase};

#[derive(Debug, Clone, Copy)]
pub struct Buffer {
    pub inner: vk::Buffer,
    pub mem: vk::DeviceMemory,
}

impl Buffer {
    pub fn create(base: &VkBase, size: u64, usage: vk::BufferUsageFlags, properties: vk::MemoryPropertyFlags) -> Self {
        let buffer_info = vk::BufferCreateInfo {
            size,
            usage,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
    
        let buffer = unsafe { base.device.create_buffer(&buffer_info, None).unwrap_unchecked() };
        let mem_requirements = unsafe { base.device.get_buffer_memory_requirements(buffer) };

        let alloc_flags_info = vk::MemoryAllocateFlagsInfo {
            flags: if usage.contains(vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS) {vk::MemoryAllocateFlags::DEVICE_ADDRESS} else {MemoryAllocateFlags::empty()},  // Aktiviert die Nutzung von Geräteadressen
            ..Default::default()
        };
    
        let alloc_info = vk::MemoryAllocateInfo {
            allocation_size: mem_requirements.size,
            memory_type_index: find_memory_type(base, mem_requirements.memory_type_bits, properties),
            p_next: &alloc_flags_info as *const _ as *const _,
            ..Default::default()
        };
    
        let mem = unsafe { base.device.allocate_memory(&alloc_info, None).unwrap() };
    
        unsafe { base.device.bind_buffer_memory(buffer, mem, 0).unwrap_unchecked() };
        Self { inner: buffer, mem }
    }

    pub fn null() -> Self {
        Self { inner: vk::Buffer::null(), mem: vk::DeviceMemory::null() }
    }

    pub fn device_local(base: &VkBase, command_pool: &vk::CommandPool, stride: u64, len: u64, data: *const u8, usage: vk::BufferUsageFlags) -> Self {
        let buffer_size = stride * len;
        let staging_buffer = Self::create(base, buffer_size, usage | vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        let mapped_memory = staging_buffer.map_memory(&base.device, buffer_size);
        unsafe {
            std::ptr::copy_nonoverlapping(data, mapped_memory as _, buffer_size as usize);
            staging_buffer.unmap_memory(&base.device);
        };

        let device_local_buffer = Self::create(base, buffer_size, usage | vk::BufferUsageFlags::TRANSFER_DST, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let cmd_buf = SinlgeTimeCommands::begin(&base, &command_pool);
        staging_buffer.copy(&device_local_buffer, base, buffer_size, cmd_buf);
        SinlgeTimeCommands::end(base, command_pool, cmd_buf);

        staging_buffer.destroy(&base.device);

        device_local_buffer
    }

    #[inline]
    pub fn device_local2<T>(base: &VkBase, command_pool: &vk::CommandPool, data: &[T], usage: vk::BufferUsageFlags) -> Self {
        let buffer_size = data.len() as u64 * size_of::<T>() as u64;
        let staging_buffer = Self::create(base, buffer_size, usage | vk::BufferUsageFlags::TRANSFER_SRC, vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT);

        let mapped_memory = staging_buffer.map_memory(&base.device, buffer_size);
        unsafe {
            std::ptr::copy_nonoverlapping(data.as_ptr(), mapped_memory as _, data.len());
        };
        staging_buffer.unmap_memory(&base.device);

        let device_local_buffer = Self::create(base, buffer_size, usage | vk::BufferUsageFlags::TRANSFER_DST, vk::MemoryPropertyFlags::DEVICE_LOCAL);

        let cmd_buf = SinlgeTimeCommands::begin(&base, &command_pool);
        staging_buffer.copy(&device_local_buffer, base, buffer_size, cmd_buf);
        SinlgeTimeCommands::end(base, command_pool, cmd_buf);

        staging_buffer.destroy(&base.device);

        device_local_buffer
    }

    pub fn copy(&self, dst_buffer: &Self, base: &VkBase, size: vk::DeviceSize, cmd_buf: vk::CommandBuffer) {

        let copy_region = vk::BufferCopy {
            src_offset: 0,
            dst_offset: 0,
            size
        };
    
        unsafe { base.device.cmd_copy_buffer(cmd_buf, self.inner, dst_buffer.inner, &[copy_region]) };
    }

    #[inline]
    pub fn map_memory(&self, device: &ash::Device, buffer_size: u64) -> *const c_void {
        unsafe { device.map_memory(self.mem, 0, buffer_size, vk::MemoryMapFlags::empty()).unwrap() }
    }

    #[inline]
    pub fn unmap_memory(&self, device: &ash::Device) {
        unsafe { device.unmap_memory(self.mem) };
    }

    #[inline]
    pub fn get_device_addr(&self, device: &ash::Device) -> vk::DeviceOrHostAddressKHR {
        let device_address_info = vk::BufferDeviceAddressInfo {
            buffer: self.inner,
            ..Default::default()
        };

        let device_address = unsafe {
            device.get_buffer_device_address(&device_address_info)
        };
    
        vk::DeviceOrHostAddressKHR {
            device_address,
        }
    }

    #[inline]
    pub fn get_device_addr_u64(&self, device: &ash::Device) -> u64 {
        let device_address_info = vk::BufferDeviceAddressInfo {
            buffer: self.inner,
            ..Default::default()
        };

        unsafe { device.get_buffer_device_address(&device_address_info) }
    }

    #[inline]
    pub fn get_device_addr_const(&self, device: &ash::Device) -> vk::DeviceOrHostAddressConstKHR {
        let device_address_info = vk::BufferDeviceAddressInfo {
            buffer: self.inner,
            ..Default::default()
        };

        let device_address = unsafe {
            device.get_buffer_device_address(&device_address_info)
        };
    
        vk::DeviceOrHostAddressConstKHR {
            device_address,
        }
    }

    #[inline]
    pub fn destroy(&self, device: &ash::Device) {
        unsafe {
            device.destroy_buffer(self.inner, None);
            device.free_memory(self.mem, None);
        }
    }
}

pub fn find_memory_type(base: &VkBase, type_filter: u32, properties: vk::MemoryPropertyFlags) -> u32 {
    let mem_properties = unsafe { base.instance.get_physical_device_memory_properties(base.physical_device) };

    for i in 0..mem_properties.memory_type_count {
        if (type_filter & (1 << i) != 0) && (mem_properties.memory_types[i as usize].property_flags & properties) == properties {
            return i;
        }
    }

    panic!("Can not find memory type!");
}