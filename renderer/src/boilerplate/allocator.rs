use ash::{
    vk::{self, BufferCreateInfo, BufferUsageFlags, ImageCreateInfo, PhysicalDevice},
    Device, Instance,
};
use vk_mem::{Alloc, AllocationCreateInfo};

use gpu_info::BufferGpuInfo;

use crate::primitives::AllocatedImage;
use asset_manager::Vertex;

pub struct Allocator {
    allocator: vk_mem::Allocator,
}

impl Allocator {
    pub fn new(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        device: &Device,
    ) -> Result<Allocator, String> {
        let create_info = vk_mem::AllocatorCreateInfo::new(instance, device, *physical_device);

        match vk_mem::Allocator::new(create_info) {
            Ok(allocator) => Ok(Allocator { allocator }),
            Err(e) => Err("Failed to create allocator: ".to_owned() + &e.to_string()),
        }
    }

    pub fn create_image(
        &self,
        image_create_info: &ImageCreateInfo,
        allocation_create_info: &AllocationCreateInfo,
    ) -> Result<AllocatedImage, String> {
        match unsafe {
            self.allocator
                .create_image(image_create_info, allocation_create_info)
        } {
            Ok((image, allocation)) => Ok(AllocatedImage { image, allocation }),
            Err(e) => Err("Failed to create image: ".to_owned() + &e.to_string()),
        }
    }

    pub fn destroy_image(&self, image: &mut AllocatedImage) {
        unsafe {
            self.allocator
                .destroy_image(image.image, &mut image.allocation);
        }
    }

    pub fn create_vertex_buffer(&self, vertices: &[Vertex]) -> BufferGpuInfo {
        let (buffer, mut allocation) = unsafe {
            self.allocator
                .create_buffer(
                    &BufferCreateInfo::default()
                        .size(std::mem::size_of_val(vertices) as u64)
                        .usage(BufferUsageFlags::VERTEX_BUFFER),
                    &AllocationCreateInfo {
                        required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
                        flags: vk_mem::AllocationCreateFlags::MAPPED
                            | vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE,
                        usage: vk_mem::MemoryUsage::Auto,
                        ..Default::default()
                    },
                )
                .unwrap()
        };

        let memory_handle = unsafe { self.allocator.map_memory(&mut allocation).unwrap() };
        unsafe {
            std::ptr::copy_nonoverlapping(
                vertices.as_ptr() as *const u8,
                memory_handle,
                std::mem::size_of_val(vertices),
            );
        }
        unsafe { self.allocator.unmap_memory(&mut allocation) };

        BufferGpuInfo { buffer, allocation }
    }

    pub fn destroy_buffer(&self, buffer: &mut BufferGpuInfo) {
        unsafe {
            self.allocator
                .destroy_buffer(buffer.buffer, &mut buffer.allocation);
        }
    }
}
