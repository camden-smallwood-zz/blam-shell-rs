use crate::commands::CommandContext;

pub struct CommandContextStack {
    pub stack: Vec<Box<CommandContext>>
}

impl CommandContextStack {
    pub fn new() -> Self {
        Self { stack: vec![] }
    }

    pub fn get_context(&mut self) -> Option<&mut Box<CommandContext>> {
        self.stack.last_mut()
    }

    pub fn get_path(&self) -> String {
        self.stack
            .iter()
            .map(|x| x.name.to_string())
            .collect::<Vec<String>>()
            .join("\\")
    }

    pub fn push(&mut self, context: Box<CommandContext>) {
        self.stack.push(context)
    }

    pub fn pop(&mut self) -> Option<Box<CommandContext>> {
        self.stack.pop()
    }
}