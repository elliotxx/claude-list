//! Output formatters for different display modes

pub mod compact;
pub mod detailed;
pub mod json;

use crate::info::ClaudeInfo;
use std::io::Write;

pub trait Formatter {
    fn format(&self, info: &ClaudeInfo, output: &mut dyn Write) -> std::io::Result<()>;
}
