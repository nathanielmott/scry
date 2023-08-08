use clap::Parser;
use eyre::ErrReport;

mod contents;
mod metadata;

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
fn main() -> eyre::Result<(), ErrReport> {
    let cli = Cli::parse();
    let file = cli.file.as_deref().unwrap();

    if cli.metadata == true {
        let data = metadata::FileData::new(file)?;
        println!(
            "User:     {}\nPath:     {}\nBytes:    {}\nCreated:  {}\nModified: {}\nAccessed: {}",
            data.user, data.path, data.size, data.created, data.modified, data.accessed
        );
    }

    if cli.content == true {
        let analysis = contents::FileAnalysis::new(file)?;
        println!(
            "Words: {}\nLines: {}\nGrafs: {}",
            analysis.word_count, analysis.line_count, analysis.paragraph_count
        );
    }
    Ok(())
}
