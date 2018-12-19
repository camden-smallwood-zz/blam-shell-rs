use blam::cache::CacheContext;
use crate::commands::Command;
use std::{cell::RefCell, fs::File, isize, io::{self, Error, ErrorKind, Read, Seek, SeekFrom, Write}, rc::Rc};

pub struct ExtractTagCommand {
    pub cache_context: Rc<RefCell<CacheContext>>
}

impl ExtractTagCommand {
    pub fn new(cache_context: Rc<RefCell<CacheContext>>) -> Self {
        Self { cache_context }
    }
}

impl Command for ExtractTagCommand {
    fn get_name(&self) -> &'static str { "ExtractTag" }
    fn is_shared(&self) -> bool { true }

    fn execute(&mut self, args: Vec<String>) -> io::Result<()> {
        if args.len() != 2 {
            Err(Error::new(ErrorKind::InvalidInput, "Invalid arguments supplied. ExtractTag <Tag> <Path>"))
        } else {
            match if args[0].len() >= 2 && &args[0].as_str()[0..2] == "0x" {
                isize::from_str_radix(&args[0].as_str()[2..], 16)
            } else {
                isize::from_str_radix(args[0].as_str(), 16)
            } {
                Err(_) => {
                    Err(Error::new(ErrorKind::InvalidInput, format!("Invalid tag index supplied: {}", args[0]).to_string()))
                },
                Ok(tag_index) => {
                    if tag_index < 0 || tag_index >= self.cache_context.borrow().tag_cache.get_tag_count() {
                        Err(Error::new(ErrorKind::InvalidInput, format!("Invalid tag index supplied: {}", args[0]).to_string()))
                    } else {
                        let mut file = File::create(args[1].as_str())?;
                        
                        let tag_cache = &mut self.cache_context.borrow_mut().tag_cache;
                        let tag = &tag_cache[tag_index as usize];
                        
                        let mut data = vec![0u8; tag.header.unwrap().size as usize];
                        tag_cache.file.seek(SeekFrom::Start(tag.offset.unwrap()))?;
                        tag_cache.file.read_exact(data.as_mut_slice())?;
                        
                        file.write_all(data.as_slice())
                    }
                }
            }
        }
    }
}