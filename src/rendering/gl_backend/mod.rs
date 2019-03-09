use super::*;

pub struct GlBackend {
    
}

impl GlBackend {
    pub fn new() -> GlBackend {
        GlBackend {

        }
    }
}

impl Backend for GlBackend {
    fn execute(command_buffer: CommandBuffer) {
        for command in command_buffer.get_commands() {
            match command {
                Command::Swap => {
                    
                }
            }
        }
    }
}
