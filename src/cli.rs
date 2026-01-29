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

    /// Output mode: compact (default), detailed (-l), full (-ll)
    #[arg(short, long, value_enum)]
    pub output: Option<OutputMode>,

    /// Filter by component type
    #[arg(long)]
    pub plugins: bool,
    #[arg(long)]
    pub skills: bool,
    #[arg(long)]
    pub sessions: bool,
    #[arg(long)]
    pub mcp: bool,
    #[arg(long)]
    pub hooks: bool,
    #[arg(long)]
    pub agents: bool,
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
    /// Full: all information including paths
    Full,
}
