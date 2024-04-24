use ash::{
    vk::{
        self, Extent3D, Format, Image, ImageCreateInfo, ImageTiling, ImageType, ImageUsageFlags,
        ImageView, ImageViewCreateInfo, ImageViewType, SampleCountFlags, SwapchainCreateInfoKHR,
        SwapchainKHR,
    },
    Device, Instance,
};
use log::warn;
use vk_mem::{Alloc, AllocationCreateInfo, Allocator};

use super::{AllocatedImage, Queue, Surface};

pub struct Swapchain {
    device: Device,
    loader: ash::khr::swapchain::Device,
    pub swapchain: SwapchainKHR,
    pub extent: vk::Extent2D,
    pub image_format: Format,
    _images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    depth_image: AllocatedImage,
    pub depth_image_view: ImageView,
}

impl Swapchain {
    pub fn new(
        instance: &Instance,
        device: &Device,
        allocator: &Allocator,
        surface: &Surface,
        queue: &Queue,
    ) -> Result<Swapchain, String> {
        let graphics_queue_indices = [queue.main_queue_index];

        let extent = surface.capabilities.current_extent;

        let min_image_count = surface.capabilities.min_image_count;
        let mut max_image_count = surface.capabilities.max_image_count;
        if max_image_count == 0 {
            max_image_count = 3;
        }

        let create_info = SwapchainCreateInfoKHR::default()
            .surface(surface.surface)
            .image_format(surface.formats.first().unwrap().format)
            .image_color_space(surface.formats.first().unwrap().color_space)
            // Try for 3 images, but otherwise pick something in the range the swapchain allows
            .min_image_count(3.max(min_image_count).min(max_image_count))
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .image_extent(extent)
            .pre_transform(surface.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO)
            .queue_family_indices(&graphics_queue_indices);

        let loader = ash::khr::swapchain::Device::new(instance, device);

        let swapchain = match unsafe { loader.create_swapchain(&create_info, None) } {
            Ok(swapchain) => swapchain,
            Err(e) => return Err("Failed to create swapchain: ".to_owned() + &e.to_string()),
        };

        let images = match unsafe { loader.get_swapchain_images(swapchain) } {
            Ok(images) => images,
            Err(e) => return Err("Failed to get swapchain images: ".to_owned() + &e.to_string()),
        };

        let image_format = surface.formats.first().unwrap().format;

        let mut image_views: Vec<vk::ImageView> = Vec::with_capacity(images.len());

        for image in &images {
            let subresource_range = vk::ImageSubresourceRange::default()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1);

            let image_view_create_info = vk::ImageViewCreateInfo::default()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(image_format)
                .subresource_range(subresource_range);

            let image_view = match unsafe {
                device.create_image_view(&image_view_create_info, None)
            } {
                Ok(image_view) => image_view,
                Err(e) => return Err("Failed to create image view: ".to_owned() + &e.to_string()),
            };

            image_views.push(image_view);
        }

        let depth_image_create_info = ImageCreateInfo::default()
            .image_type(ImageType::TYPE_2D)
            .format(Format::D32_SFLOAT)
            .extent(Extent3D {
                width: extent.width,
                height: extent.height,
                depth: 1,
            })
            .mip_levels(1)
            .array_layers(1)
            .samples(SampleCountFlags::TYPE_1)
            .tiling(ImageTiling::OPTIMAL)
            .usage(ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT);

        // TODO: This is marked as deprecated in the vk_mem crate, but the replacement is not yet implemented
        //       GpuOnly is the only option for now that is working for me
        #[allow(deprecated)]
        let depth_image_allocation_create_info = AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::GpuOnly,
            required_flags: vk::MemoryPropertyFlags::DEVICE_LOCAL,
            ..Default::default()
        };

        let (image, allocation) = unsafe {
            allocator.create_image(
                &depth_image_create_info,
                &depth_image_allocation_create_info,
            )
        }
        .unwrap();

        let depth_image = AllocatedImage { image, allocation };

        let depth_image_view_create_info = ImageViewCreateInfo::default()
            .view_type(ImageViewType::TYPE_2D)
            .image(depth_image.image)
            .format(Format::D32_SFLOAT)
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::DEPTH)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );

        let depth_image_view =
            unsafe { device.create_image_view(&depth_image_view_create_info, None) }.unwrap();

        Ok(Swapchain {
            device: device.clone(),
            loader,
            swapchain,
            extent,
            image_format,
            _images: images,
            image_views,
            depth_image,
            depth_image_view,
        })
    }

    pub fn acquire_next_image(&self, semaphore: vk::Semaphore) -> Result<(u32, bool), String> {
        match unsafe {
            self.loader
                .acquire_next_image(self.swapchain, 1000000000, semaphore, vk::Fence::null())
        } {
            Ok((image_index, is_suboptimal)) => Ok((image_index, is_suboptimal)),
            Err(e) => Err("Failed to acquire next image: ".to_owned() + &e.to_string()),
        }
    }

    pub fn present(&self, queue: &Queue, image_index: u32, wait_semaphores: &[vk::Semaphore]) {
        let swapchains = [self.swapchain];
        let image_indices = [image_index];

        let present_info = vk::PresentInfoKHR::default()
            .wait_semaphores(wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&image_indices);

        match unsafe { self.loader.queue_present(queue.main_queue, &present_info) } {
            Ok(_) => {}
            Err(e) => {
                warn!("Failed to present image: {}", e);
            }
        }
    }

    pub fn free(&mut self, allocator: &mut Allocator) {
        unsafe {
            self.device.destroy_image_view(self.depth_image_view, None);

            allocator.destroy_image(self.depth_image.image, &mut self.depth_image.allocation);

            for image_view in &self.image_views {
                self.device.destroy_image_view(*image_view, None);
            }

            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }
}
