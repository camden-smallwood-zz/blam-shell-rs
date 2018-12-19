use std::io;

pub trait Command {
    fn get_name(&self) -> &'static str;
    fn is_shared(&self) -> bool;
    fn execute(&mut self, args: Vec<String>) -> io::Result<()>;
}