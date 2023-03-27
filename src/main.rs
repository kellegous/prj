use clap::Parser;
use prj::{Command, ProjectDir};
use std::error::Error;
use std::path::PathBuf;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn get_documents_dir() -> Result<PathBuf, Box<dyn Error>> {
    let p = home::home_dir()
        .ok_or("Could not find home directory")?
        .join("Documents");
    Ok(p)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dir = ProjectDir::for_user()?;

    args.command.run(&dir)?;
    Ok(())
}
