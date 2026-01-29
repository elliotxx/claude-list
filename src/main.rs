use anyhow::Result;
use clap::Parser;
use std::process;

use claude_list::cli::Args;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {:#}", e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();
    println!("Config directory: {:?}", args.config);
    Ok(())
}
