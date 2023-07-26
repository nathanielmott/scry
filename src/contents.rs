use eyre::ErrReport;
use std::fs;

#[derive(Debug)]
pub struct FileAnalysis {
    pub word_count: usize,
    pub line_count: usize,
    pub paragraph_count: usize,
}

impl FileAnalysis {
    pub fn new(file: &str) -> eyre::Result<FileAnalysis, ErrReport> {
        let contents: String = fs::read_to_string(file)?;
        let words: Vec<&str> = contents.split_ascii_whitespace().collect();
        let word_count = words.len();
        let lines: Vec<&str> = contents.lines().collect();
        let line_count = lines.len();
        let grafs: Vec<&str> = contents.split("\n\n").collect();
        let paragraph_count = grafs.len();

        Ok(FileAnalysis {
            word_count,
            line_count,
            paragraph_count,
        })
    }
}
