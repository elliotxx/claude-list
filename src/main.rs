use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process;

use claude_list::cli::{Args, OutputMode};
use claude_list::formatters::compact::format_compact;
use claude_list::parsers::{filter_components, parse_all};

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Determine config directory
    let config_dir = match args.config {
        Some(path) => path,
        None => {
            // Default to ~/.claude or CLAUDE_DIR env var
            let home = env::var("HOME").context("HOME not set")?;
            PathBuf::from(home).join(".claude")
        }
    };

    // Check if config directory exists
    if !config_dir.exists() {
        anyhow::bail!("Directory not found: {}", config_dir.display());
    }

    // Parse all components
    let info = parse_all(config_dir)?;

    // Filter based on flags
    let info = filter_components(
        info,
        args.plugins,
        args.skills,
        args.sessions,
        args.mcp,
        args.hooks,
        args.agents,
        args.commands,
    );

    // Output based on mode
    if args.json {
        let json = serde_json::to_string_pretty(&info)?;
        println!("{}", json);
    } else {
        let mode = args.output.unwrap_or(OutputMode::Compact);
        match mode {
            OutputMode::Compact => {
                format_compact(&info, &mut std::io::stdout())?;
            }
            OutputMode::Detailed => {
                // For now, use compact format
                // Detailed formatter to be implemented in User Story 2
                format_compact(&info, &mut std::io::stdout())?;
            }
            OutputMode::Full => {
                // For now, use compact format
                // Full formatter to be implemented in User Story 2
                format_compact(&info, &mut std::io::stdout())?;
            }
        }
    }

    Ok(())
}
