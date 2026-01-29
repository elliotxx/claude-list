//! CLI argument definitions

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "claude-list")]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Custom .claude directory path
    #[arg(short, long, value_name = "PATH")]
    pub config: Option<PathBuf>,

    /// Output mode: compact (default), detailed (-l)
    #[arg(short, long, value_enum)]
    pub output: Option<OutputMode>,

    /// Filter to show only plugins
    #[arg(long)]
    pub plugins: bool,
    /// Filter to show only skills
    #[arg(long)]
    pub skills: bool,
    /// Filter to show only sessions
    #[arg(long)]
    pub sessions: bool,
    /// Filter to show only MCP servers
    #[arg(long)]
    pub mcp: bool,
    /// Filter to show only hooks
    #[arg(long)]
    pub hooks: bool,
    /// Filter to show only agents
    #[arg(long)]
    pub agents: bool,
    /// Filter to show only commands
    #[arg(long)]
    pub commands: bool,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutputMode {
    /// Compact: name only
    Compact,
    /// Detailed: name, version, source
    Detailed,
}
