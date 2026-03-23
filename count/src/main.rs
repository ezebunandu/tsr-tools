use std::{
    env,
    fs::File,
    io::{BufReader, Error, Result},
};

use count::count_lines;

fn main() -> Result<()> {
    let path = env::args()
        .nth(1)
        .ok_or(Error::other("Usage: count <FILE>"))?;
    let file = File::open(&path)?;
    let file = BufReader::new(file);
    let lines = count_lines(file)?;
    println!("{lines} lines");
    Ok(())
}
