#[derive(Debug)]
pub enum Command {
    Swap,
}

#[derive(Debug)]
pub struct CommandBuffer {
    commands: Vec<Command>,
}

impl CommandBuffer {
    pub fn new() -> CommandBuffer {
        CommandBuffer {
            commands: Vec::new(),
        }
    }

    pub fn swap(&mut self) {
        self.commands.push(Command::Swap);
    }

    pub fn get_commands(self) -> Vec<Command> {
        self.commands
    }
}
