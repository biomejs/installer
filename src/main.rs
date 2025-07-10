use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::process;

use crate::commands::Commands;

mod commands;
mod downloader;
mod installer;
mod platform;

#[derive(Parser)]
#[command(name = "biome-installer")]
#[command(about = "A cross-platform installer for Biome")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: commands::Commands,
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{} {}", "✘".red(), err.to_string().red());

        for cause in err.chain().skip(1) {
            eprintln!("{} {}", "→ caused by:".dimmed(), cause.to_string().dimmed());
        }

        process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Install(cmd) => cmd.handle()?,
    }

    Ok(())
}
