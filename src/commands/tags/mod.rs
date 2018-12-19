mod extract_tag;
mod import_tag;

use blam::cache::CacheContext;
use crate::commands::CommandContext;
use std::{cell::RefCell, rc::Rc};

pub fn create_context(cache_context: Rc<RefCell<CacheContext>>, parent: Option<Box<CommandContext>>) -> CommandContext {
    let mut context = CommandContext::new("tags".to_string(), parent);
    context.add_command(Box::new(extract_tag::ExtractTagCommand::new(Rc::clone(&cache_context))));
    context.add_command(Box::new(import_tag::ImportTagCommand::new(Rc::clone(&cache_context))));

    //
    // TODO: add commands to tags_context here
    //

    context
}