use ash::{
    vk::{self, Format, Image, ImageView, SwapchainCreateInfoKHR, SwapchainKHR},
    Device, Instance,
};

use super::{Queue, Surface};

pub struct Swapchain {
    device: Device,
    loader: ash::extensions::khr::Swapchain,
    swapchain: SwapchainKHR,
    pub extent: vk::Extent2D,
    pub image_format: Format,
    images: Vec<Image>,
    pub image_views: Vec<ImageView>,
}

impl Swapchain {
    pub fn new(
        instance: &Instance,
        device: &Device,
        surface: &Surface,
        queue: &Queue,
    ) -> Result<Swapchain, String> {
        let graphics_queue_indices = [queue.main_queue_index];

        let extent = surface.capabilities.current_extent;

        let create_info = SwapchainCreateInfoKHR::builder()
            .surface(surface.surface)
            .image_format(surface.formats.first().unwrap().format)
            .image_color_space(surface.formats.first().unwrap().color_space)
            // Try for 3 images, but otherwise pick something in the range the swapchain allows
            .min_image_count(
                3.max(surface.capabilities.min_image_count)
                    .min(surface.capabilities.max_image_count),
            )
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .image_extent(extent)
            .pre_transform(surface.capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(vk::PresentModeKHR::FIFO)
            .queue_family_indices(&graphics_queue_indices)
            .build();

        let loader = ash::extensions::khr::Swapchain::new(instance, device);

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
            let subresource_range = vk::ImageSubresourceRange::builder()
                .aspect_mask(vk::ImageAspectFlags::COLOR)
                .base_mip_level(0)
                .level_count(1)
                .base_array_layer(0)
                .layer_count(1)
                .build();

            let image_view_create_info = vk::ImageViewCreateInfo::builder()
                .image(*image)
                .view_type(vk::ImageViewType::TYPE_2D)
                .format(image_format)
                .subresource_range(subresource_range)
                .build();

            let image_view = match unsafe {
                device.create_image_view(&image_view_create_info, None)
            } {
                Ok(image_view) => image_view,
                Err(e) => return Err("Failed to create image view: ".to_owned() + &e.to_string()),
            };

            image_views.push(image_view);
        }

        Ok(Swapchain {
            device: device.clone(),
            loader,
            swapchain,
            extent,
            image_format,
            images,
            image_views,
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
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            for image_view in &self.image_views {
                self.device.destroy_image_view(*image_view, None);
            }

            self.loader.destroy_swapchain(self.swapchain, None);
        }
    }
}
