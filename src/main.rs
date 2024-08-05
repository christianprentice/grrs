use anyhow::{Context, Result};
use clap::Parser;
use colored::*;

mod utils;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let pattern = &args.pattern;
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(*&pattern) {
            let colored_line = utils::colorize_substring(&line, &pattern, Color::Green);
            println!("{}", colored_line);
        }
    }

    Ok(())
}
