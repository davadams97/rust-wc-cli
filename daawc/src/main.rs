use clap::Parser;
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::io::{Read};

#[derive(Parser)]
struct Cli {    
    #[arg(short = 'c')]
    count_bytes: bool,

    #[arg(short = 'l')]
    count_lines: bool,

    #[arg(short = 'w')]
    count_words: bool,
    
    #[arg(short = 'm')]
    count_characters: bool,

    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();

    if args.count_bytes {
        let file_content = fs::read(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let total_bytes = file_content.len();
        println!("Total bytes: {} {}", total_bytes, args.path.display());
    } else if args.count_lines {
        let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let reader = BufReader::new(file);
        let mut total_lines = 0;

        for _ in reader.lines() {
            total_lines += 1;
        }
        println!("Total lines: {} {}", total_lines, args.path.display());
    } else if args.count_words {
        let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let reader = BufReader::new(file);
        let mut total_words = 0;

        for line in reader.lines() {
            let line = line?;
            total_words += line.split_whitespace().count();
        }
        println!("Total words: {} {}", total_words, args.path.display());
    } else if args.count_characters {
        let mut file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let mut content = String::new();

        file.read_to_string(&mut content)?;
        println!("Total characters: {} {}", content.chars().count(), args.path.display());
    } else {
        let file = File::open(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let reader = BufReader::new(file);
        let mut total_lines = 0;
        let mut total_words = 0;

        for line in reader.lines() {
            let line = line?;
            total_words += line.split_whitespace().count();
            total_lines += 1;
        }

        let file_content = fs::read(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
        let total_bytes = file_content.len();
        println!("{} {} {} {}", total_lines, total_words, total_bytes, args.path.display());
    }

    Ok(())
}



