extern crate blam;

pub mod commands;

use blam::cache::CacheContext;
use crate::commands::CommandContextStack;
use std::{cell::RefCell, ffi::OsStr, io::{self, Write}, path::{Path, PathBuf}, rc::Rc};

fn main() -> io::Result<()> {
    let mut input = String::new();
    let mut cache_path: PathBuf;

    println!("BlamShell [{}]", env!("CARGO_PKG_VERSION"));

    loop {
        println!();
        println!("Enter the path to 'tags.dat':");
        print!("> ");
        io::stdout().flush()?;
        
        input.clear();
        let _ = io::stdin().read_line(&mut input);

        cache_path = Path::new(input.trim()).to_path_buf();

        if cache_path.exists() {
            if let Some(file_name) = cache_path.file_name() {
                if file_name == OsStr::new("tags.dat") {
                    cache_path.pop();
                    break;
                }
            }
        }
    }

    let cache_context = CacheContext::open(cache_path)?;

    let mut command_context_stack = CommandContextStack::new();
    command_context_stack.push(Box::new(commands::tags::create_context(Rc::new(RefCell::new(cache_context)), None)));

    println!();
    println!("Enter \"help\" to list available commands. Enter \"exit\" to quit.");

    loop {
        println!();
        print!("{}> ", command_context_stack.get_path().as_str());
        io::stdout().flush()?;
        
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Err(err) => {
                break Err(err)
            },
            Ok(_) => {
                let args = commands::parse(input.trim().to_string());
                if args.len() == 0 {
                    continue;
                }
                match args[0].as_str() {
                    "exit" | "quit" => {
                        break Ok(())
                    },
                    _ => {
                        match command_context_stack.get_context().unwrap().get_command((&args[0]).clone()) {
                            None => {
                                println!("Unrecognized command: {0}", args[0]);
                                println!("Use \"help\" to list available commands.");
                            },
                            Some(command) => {
                                if let Err(err) = command.execute(args.iter().skip(1).take(args.len() - 1).map(|x| x.clone()).collect()) {
                                    println!("ERROR: {}", err);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}