use anyhow::{Context, Result};
use clap::Parser;
use colored::*;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn colorize_substring(haystack: &str, needle: &str, color: Color) -> String{
    let start_index = haystack.find(needle).unwrap_or(0);

    let before_substring = &haystack[..start_index];
    let after_substring = &haystack[start_index + needle.len()..];

    let colored_needle = needle.color(color);

    format!("{}{}{}", before_substring, colored_needle, after_substring)
}

fn main() -> Result<()> {
    let args = Cli::parse();

    let pattern = &args.pattern;
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(*&pattern) {
            let colored_line = colorize_substring(&line, &pattern, Color::Green);
            println!("{}", colored_line);
        }
    }

    Ok(())
}
