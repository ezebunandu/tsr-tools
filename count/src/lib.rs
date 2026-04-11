use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use anyhow::{Context, Result};

#[derive(Default)]
pub struct Count {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
}

pub fn count(mut input: impl BufRead) -> Result<Count> {
    let mut count = Count::default();
    let mut line = String::new();
    while input.read_line(&mut line)? > 0 {
        count.words = count
            .words
            .checked_add(line.split_whitespace().count())
            .context("overflow")?;
        count.lines = count.lines.checked_add(1).context("overflow")?;
        count.bytes = count.bytes.checked_add(line.len()).context("overflow")?;

        line.clear();
    }
    Ok(count)
}

pub fn count_in_path(path: &String) -> Result<Count> {
    let file = File::open(path).with_context(|| path.clone())?;
    let file = BufReader::new(file);
    count(file).with_context(|| path.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, BufReader, Cursor, Error, Read};

    #[test]
    fn count_counts_lines_and_words_and_bytes_in_input() {
        let input = Cursor::new("word1 word2\nword3");
        let count = count(input).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 3, "wrong word count");
        assert_eq!(count.bytes, 17, "wrong byte count");
    }

    #[test]
    fn count_fn_returns_any_read_error() {
        let reader = BufReader::new(ErrorReader);
        let result = count(reader);
        assert!(result.is_err(), "no error returned");
    }

    #[test]
    fn count_in_path_fn_counts_lines_words_and_bytes_in_given_path() {
        let path = String::from("tests/data/two_lines.txt");
        let count = count_in_path(&path).unwrap();
        assert_eq!(count.lines, 2, "wrong line count");
        assert_eq!(count.words, 4, "wrong word count");
        assert_eq!(count.bytes, 13, "wrong byte count");
    }

    struct ErrorReader;

    impl Read for ErrorReader {
        fn read(&mut self, _buf: &mut [u8]) -> io::Result<usize> {
            Err(Error::other("oh no"))
        }
    }
}
