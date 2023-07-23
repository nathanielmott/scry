use chrono::prelude::*;
use clap::Parser;
use eyre::ErrReport;
use std::fs;

#[derive(Parser)]
#[command(name = "scry")]
#[command(author = "Nathaniel Mott")]
#[command(version = "0.1.0")]
struct Cli {
    ///The file we want to retrieve information about
    file: Option<String>,
    ///Gather metadata about the specified file
    #[arg(short, long)]
    metadata: bool,
    ///Analyze the specified file's contents
    #[arg(short, long)]
    content: bool,
}

// #[derive(Subcommand)]
// enum Commands {

// }

#[derive(Debug)]
struct FileData {
    size: u64,
    created: String,
    modified: String,
    accessed: String,
}

#[derive(Debug)]
struct FileAnalysis {
    word_count: usize,
    line_count: usize,
    paragraph_count: usize,
}

fn get_data(file: &str) -> eyre::Result<FileData, ErrReport> {
    let metadata = fs::metadata(&file)?;
    let size = metadata.len();
    let created: DateTime<Local> = DateTime::from(metadata.created()?);
    let modified: DateTime<Local> = DateTime::from(metadata.modified()?);
    let accessed: DateTime<Local> = DateTime::from(metadata.accessed()?);

    let formatted_created_time = format!("{}", created.format("%m/%d/%Y %H:%M"));
    let formatted_modified_time = format!("{}", modified.format("%m/%d/%Y %H:%M"));
    let formatted_accessed_time = format!("{}", accessed.format("%m/%d/%Y %H:%M"));

    Ok(FileData {
        size,
        created: formatted_created_time,
        modified: formatted_modified_time,
        accessed: formatted_accessed_time,
    })
}

fn analyze_file(file: &str) -> eyre::Result<FileAnalysis, ErrReport> {
    let contents: String = fs::read_to_string(file)?;
    let whitespaced_words: Vec<&str> = contents.split(" ").collect();
    let word_count = whitespaced_words.len();
    let split_lines: Vec<&str> = contents.split("\n").collect();
    let line_count = split_lines.len();
    let split_grafs: Vec<&str> = contents.split("\n\n").collect();
    let paragraph_count = split_grafs.len();

    Ok(FileAnalysis {
        word_count,
        line_count,
        paragraph_count,
    })
}

fn main() -> eyre::Result<(), ErrReport> {
    let cli = Cli::parse();
    let file = cli.file.as_deref().unwrap();

    if cli.metadata == true {
        let data = get_data(file)?;
        println!(
            "Size:     {} bytes\nCreated:  {}\nModified: {}\nAccessed: {}\n",
            data.size, data.created, data.modified, data.accessed
        );
    }

    if cli.content == true {
        let analysis = analyze_file(file)?;
        println!(
            "Words:      {}\nLines:      {}\nParagraphs: {}\n",
            analysis.word_count, analysis.line_count, analysis.paragraph_count
        );
    }
    Ok(())
}
