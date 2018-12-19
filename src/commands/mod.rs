pub mod tags;

mod command;
mod command_context;
mod command_context_stack;

pub use self::command::*;
pub use self::command_context::*;
pub use self::command_context_stack::*;

pub fn parse(input: String) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    let mut string = String::new();
    let mut start: isize = -1;
    let mut quoted = false;
    for (i, c) in input.chars().enumerate() {
        match c {
            ' ' => {
                if quoted && start == -1 {
                    start = i as isize;
                } else {
                    if start != -1 {
                        string.push_str(input.chars().skip(start as usize).take(i - start as usize).collect::<String>().as_str());
                    }
                    if string.len() > 0 {
                        result.push(string.as_str().to_string());
                    }
                    string.clear();
                    start = -1;
                }
            },
            '"' => {
                quoted = !quoted;
                if start != -1 {
                    string.push_str(input.chars().skip(start as usize).take(i - start as usize).collect::<String>().as_str());
                }
                start = -1;
            },
            _ => {
                if start == -1 {
                    start = i as isize;
                }
            }
        }
    }
    if start != -1 {
        string.push_str(input.chars().skip(start as usize).take(input.len() - start as usize).collect::<String>().as_str());
    }
    if string.len() > 0 {
        result.push(string);
    }
    result
}