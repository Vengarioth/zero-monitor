use super::*;

pub trait Backend {
    fn execute(command_buffer: CommandBuffer);
}
