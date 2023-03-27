use clap::Parser;
use prj::{Command, ProjectDir};
use std::error::Error;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let dir = ProjectDir::for_user()?;

    args.command.run(&dir)?;
    Ok(())
}
