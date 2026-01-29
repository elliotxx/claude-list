//! Parsers for different component types

pub mod plugins;
pub mod skills;
pub mod sessions;
pub mod mcp;
pub mod hooks;
pub mod agents;

use crate::error::Result;
use std::path::Path;

pub trait Parser {
    fn parse(&self, base_path: &Path) -> Result<()>;
}
