use ash::{
    vk::{
        self, CommandBufferResetFlags, CommandBufferUsageFlags, Fence, PipelineBindPoint,
        PipelineLayout, PipelineStageFlags, RenderPassBeginInfo, Semaphore, ShaderStageFlags,
        SubmitInfo, SubpassContents,
    },
    Device,
};
use log::error;
use serde::Serialize;

use super::{Pipeline, Queue};

pub struct CommandManager {
    device: Device,
    queue: Queue,
    main_command_pool: vk::CommandPool,
    transfer_command_pool: vk::CommandPool,
    main_command_buffer: vk::CommandBuffer,
    _transfer_command_buffer: vk::CommandBuffer,
}

impl CommandManager {
    pub fn new(device: &Device, queue: &Queue) -> Result<CommandManager, String> {
        let mut pool_create_info = vk::CommandPoolCreateInfo::default()
            .queue_family_index(queue.main_queue_index)
            .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

        let main_command_pool = match unsafe { device.create_command_pool(&pool_create_info, None) }
        {
            Ok(pool) => pool,
            Err(_) => return Err("Failed to create command pool".to_string()),
        };

        let transfer_command_pool = if queue.main_queue_index == queue.transfer_only_queue_index {
            main_command_pool
        } else {
            pool_create_info.queue_family_index = queue.transfer_only_queue_index;

            match unsafe { device.create_command_pool(&pool_create_info, None) } {
                Ok(pool) => pool,
                Err(_) => return Err("Failed to create command pool".to_string()),
            }
        };

        let mut command_buffer_allocate_info = vk::CommandBufferAllocateInfo::default()
            .command_pool(main_command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);

        let main_command_buffer =
            match unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) } {
                Ok(buffer) => buffer[0],
                Err(_) => return Err("Failed to allocate command buffer".to_string()),
            };

        command_buffer_allocate_info.command_pool = transfer_command_pool;

        let _transfer_command_buffer =
            match unsafe { device.allocate_command_buffers(&command_buffer_allocate_info) } {
                Ok(buffer) => buffer[0],
                Err(_) => return Err("Failed to allocate command buffer".to_string()),
            };

        Ok(CommandManager {
            device: device.clone(),
            queue: queue.clone(),
            main_command_pool,
            transfer_command_pool,
            main_command_buffer,
            _transfer_command_buffer,
        })
    }

    pub fn begin_main_command_buffer(&self) {
        match unsafe {
            self.device
                .reset_command_buffer(self.main_command_buffer, CommandBufferResetFlags::empty())
        } {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to reset command buffer");
                return;
            }
        }

        let begin_info =
            vk::CommandBufferBeginInfo::default().flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        unsafe {
            self.device
                .begin_command_buffer(self.main_command_buffer, &begin_info)
                .expect("Failed to begin command buffer");
        }
    }

    pub fn end_main_command_buffer(&self) -> Result<(), String> {
        match unsafe { self.device.end_command_buffer(self.main_command_buffer) } {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to end command buffer".to_string()),
        }
    }

    pub fn begin_render_pass(&self, render_pass_begin_info: &RenderPassBeginInfo) {
        unsafe {
            self.device.cmd_begin_render_pass(
                self.main_command_buffer,
                render_pass_begin_info,
                SubpassContents::INLINE,
            )
        };
    }

    pub fn end_render_pass(&self) {
        unsafe { self.device.cmd_end_render_pass(self.main_command_buffer) };
    }

    pub fn submit_main_command_buffer(
        &self,
        wait_semaphores: &[Semaphore],
        signal_semaphores: &[Semaphore],
        fence: Fence,
    ) {
        let submit_command_buffers = [self.main_command_buffer];

        let pipeline_stage_flags = [PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];

        let submit_info = SubmitInfo::default()
            .wait_dst_stage_mask(&pipeline_stage_flags)
            .wait_semaphores(wait_semaphores)
            .signal_semaphores(signal_semaphores)
            .command_buffers(&submit_command_buffers);

        let submit_infos = [submit_info];

        match unsafe {
            self.device
                .queue_submit(self.queue.main_queue, &submit_infos, fence)
        } {
            Ok(_) => {}
            Err(_) => {
                error!("Failed to submit command buffer");
            }
        }
    }

    pub fn bind_pipeline(&self, pipeline: &Pipeline) {
        unsafe {
            self.device.cmd_bind_pipeline(
                self.main_command_buffer,
                PipelineBindPoint::GRAPHICS,
                pipeline.pipeline,
            )
        };
    }

    pub fn bind_vertex_buffers(&self, first_binding: u32, buffers: &[vk::Buffer], offsets: &[u64]) {
        unsafe {
            self.device.cmd_bind_vertex_buffers(
                self.main_command_buffer,
                first_binding,
                buffers,
                offsets,
            )
        };
    }

    pub fn push_constants<T: Serialize>(&self, layout: PipelineLayout, constants: T) {
        let bytes = bincode::serialize(&constants).unwrap();

        unsafe {
            self.device.cmd_push_constants(
                self.main_command_buffer,
                layout,
                ShaderStageFlags::VERTEX,
                0,
                &bytes,
            )
        }
    }

    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            self.device.cmd_draw(
                self.main_command_buffer,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
    }
}

impl Drop for CommandManager {
    fn drop(&mut self) {
        unsafe {
            self.device
                .destroy_command_pool(self.main_command_pool, None);
            if self.main_command_pool != self.transfer_command_pool {
                self.device
                    .destroy_command_pool(self.transfer_command_pool, None);
            }
        }
    }
}
