use anyhow::Result;
use clap::Parser;

use count::count_in_path;

#[derive(Parser)]
/// Counts lines or words in the specified files
struct Args {
    /// Counts words instead of lines
    #[arg(short, long)]
    word: bool,
    /// Counts bytes instead of lines
    #[arg(short, long)]
    byte: bool,
    /// Files to be counted
    #[arg(required = true)]
    files: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    for path in args.files {
        let count = count_in_path(&path)?;
        if args.word {
            println!("{path}: {} words", count.words);
        }
        if args.byte {
            println!("{path}: {} bytes", count.bytes);
        }
        if !args.word && !args.byte {
            println!("{path}: {} lines", count.lines);
        }
    }
    Ok(())
}
