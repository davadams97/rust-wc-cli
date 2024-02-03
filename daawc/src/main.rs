use clap::Parser;
use anyhow::{Context, Result};
use std::fs;

#[derive(Parser)]
struct Cli {    
    #[arg(short = 'c')]
    path: std::path::PathBuf,
}

fn main() -> Result<()>{
    let args = Cli::parse();

    let file_content = fs::read(&args.path).with_context(|| format!("could not read file `{}`", args.path.display()))?;
    let total_bytes = file_content.len();

    println!("Total bytes: {}", total_bytes);

    Ok(())
}


