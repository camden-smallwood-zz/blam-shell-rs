use crate::commands::Command;

pub struct CommandContext {
    pub name: String,
    pub parent: Option<Box<CommandContext>>,
    pub commands: Vec<Box<dyn Command>>
}

impl CommandContext {
    pub fn new(name: String, parent: Option<Box<CommandContext>>) -> Self {
        Self { name, parent, commands: vec![] }
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        if (&self.commands).into_iter().find(|&x| x.get_name() == command.get_name()).is_none() {
            self.commands.push(command);
        }
    }

    pub fn get_command(&mut self, name: String) -> Option<&mut Box<dyn Command>> {
        for command in self.commands.iter_mut() {
            if (*command).get_name() == name {
                return Some(command);
            }
        }
        None
    }
}