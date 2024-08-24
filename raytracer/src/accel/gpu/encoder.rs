use wgpu;

use super::instance::Instance;

pub struct CommandEncoder(pub(super) wgpu::CommandEncoder);

impl Instance {
    pub fn create_command_encoder(&self) -> CommandEncoder {
        let desc = wgpu::CommandEncoderDescriptor {
            label: None
        };
        let encoder = self.device.create_command_encoder(&desc);
        CommandEncoder(encoder)
    }

    pub fn execute(&self, encoder: CommandEncoder) {
        self.queue.submit(Some(encoder.0.finish()));
    }
}