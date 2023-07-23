use chrono::prelude::*;
use clap::Parser;
use eyre::ErrReport;
use std::{fs, io};

#[derive(Parser)]
#[command(name = "scry")]
#[command(author = "Nathaniel Mott")]
#[command(version = "0.1.0")]
struct Cli {
    file: Option<String>,
    // command: Option<Commands>,
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

fn main() {
    let cli = Cli::parse();
    let file = cli.file.as_deref().expect("Couldn't find file.");
    let data = get_data(file);
    println!("{:?}", data);
}
