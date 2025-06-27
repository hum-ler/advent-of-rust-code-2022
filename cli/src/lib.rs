use std::fs;

use anyhow::{Result, anyhow};
use clap::Parser;

#[derive(Parser)]
struct Args {
    part: u8,

    #[arg(short = 'i', long)]
    input: Option<String>,
}

pub enum Part {
    Part1(String),
    Part2(String),
}

/// Gets the [Part] to execute.
pub fn get_part(default_input: &str) -> Result<Part> {
    let args = Args::parse();

    let path = args.input.unwrap_or(String::from(default_input));
    let input = String::from(trim_newlines(&fs::read_to_string(path)?));

    match args.part {
        1 => Ok(Part::Part1(input)),
        2 => Ok(Part::Part2(input)),
        _ => Err(anyhow!("Invalid part number: {}", args.part)),
    }
}

/// Trims newlines from the start and the end of the input string.
pub fn trim_newlines(input: &str) -> &str {
    input.trim_start_matches("\n").trim_end_matches("\n")
}
