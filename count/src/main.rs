use anyhow::{Result, bail};
use std::env;

use count::count_in_path;

fn main() -> Result<()> {
    let args: Vec<_> = env::args().skip(1).collect();
    if args.is_empty() {
        bail!("Usage: count [-w] <FILE>...");
    }
    let mut word_mode = false;
    for path in args {
        if path == "-w" {
            word_mode = true;
            continue;
        }
        let count = count_in_path(&path)?;
        if word_mode {
            println!("{path}: {} words", count.words);
        } else {
            println!("{path}: {} lines", count.lines);
        }
    }
    Ok(())
}
