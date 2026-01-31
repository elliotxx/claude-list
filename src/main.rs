use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::path::PathBuf;
use std::process;

use claude_list::cli::{Args, OutputMode};
use claude_list::formatters::compact::format_compact;
use claude_list::formatters::detailed::format_detailed;
use claude_list::output::{ColorScheme, ColorSettings};
use claude_list::parsers::{filter_components, parse_all, FilterFlags, SearchFilter};

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

    // Create search filter
    let search_filter = args.search.as_ref().map(|q| SearchFilter::new(q));

    // Create color settings (respect --no-color flag)
    let mut color_settings = ColorSettings::from_env();
    if args.no_color {
        color_settings = ColorSettings {
            enabled: false,
            force_colors: false,
        };
    }
    let color_scheme = ColorScheme::default();

    // Filter based on flags
    let filters = FilterFlags {
        plugins: args.plugins,
        skills: args.skills,
        sessions: args.sessions,
        mcp: args.mcp,
        hooks: args.hooks,
        agents: args.agents,
        commands: args.commands,
        search: search_filter,
    };
    let info = filter_components(info, filters);

    // Output based on mode
    if args.json {
        let json = serde_json::to_string_pretty(&info)?;
        println!("{}", json);
    } else {
        // --detailed or -l flag takes precedence, then --output
        let mode = if args.detailed {
            OutputMode::Detailed
        } else {
            args.output.unwrap_or(OutputMode::Compact)
        };
        match mode {
            OutputMode::Compact => {
                format_compact(
                    &info,
                    &color_scheme,
                    &color_settings,
                    &mut std::io::stdout(),
                )?;
            }
            OutputMode::Detailed => {
                // Detailed format: shows version, source, path
                format_detailed(
                    &info,
                    &color_scheme,
                    &color_settings,
                    &mut std::io::stdout(),
                )?;
            }
        }
    }

    Ok(())
}
