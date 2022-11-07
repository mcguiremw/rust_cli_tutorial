#![allow(unused)]

use clap::Parser;

#[derive(Debug)]
struct FileParseError(String);

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf
}

fn main() -> Result<(), FileParseError> {
    // let args = Cli::parse();
    let path = "data/test.txt";
    let content = std::fs::read_to_string(path)
        .map_err(|err| FileParseError(format!("Error reading `{}`: {}", path, err)))?;
    println!("file content: {}", content);
    Ok(())
}
