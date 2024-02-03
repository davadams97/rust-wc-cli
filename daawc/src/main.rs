use clap::Parser;
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Parser)]
struct Cli {    
    #[arg(short = 'c')]
    count_bytes: bool,

    #[arg(short = 'l')]
    count_lines: bool,

    #[arg(short = 'w')]
    count_words: bool,

    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();

    if args.count_bytes {
        let file_content = fs::read(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let total_bytes = file_content.len();
        println!("Total bytes: {}", total_bytes);
    } else if args.count_lines {
        let file = File::open(&args.path)?;
        let reader = BufReader::new(file);
        let mut total_lines = 0;

        for _ in reader.lines() {
            total_lines += 1;
        }
        println!("Total lines: {}", total_lines);
    } else if args.count_words {
        let file = File::open(&args.path)?;
        let reader = BufReader::new(file);
        let mut word_count = 0;

        for line in reader.lines() {
            let line = line?;
            word_count += line.split_whitespace().count();
        }
        println!("Total words: {}", word_count);
    }

    Ok(())
}


