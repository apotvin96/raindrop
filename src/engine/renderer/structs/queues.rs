use ash::Instance;

pub struct Queues {
    pub graphics_queue_index: u32,
    pub transfer_queue_index: u32,
}

impl Queues {
    pub fn new(_instance: &Instance) -> Result<Queues, String> {
        Ok(Queues {
            graphics_queue_index: 0,
            transfer_queue_index: 0,
        })
    }
}
