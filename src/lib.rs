mod list;
mod search;

use clap::Subcommand;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Subcommand)]
pub enum Command {
    List(list::Args),
    Search(search::Args),
}

impl Command {
    pub fn run(&self, dir: &ProjectDir) -> Result<(), Box<dyn Error>> {
        match self {
            Command::List(args) => list::run(dir, args)?,
            Command::Search(args) => search::run(dir, args)?,
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ProjectDir {
    pub path: PathBuf,
}

impl ProjectDir {
    pub fn for_user() -> Result<ProjectDir, Box<dyn Error>> {
        let p = home::home_dir()
            .ok_or("Could not find home directory")?
            .join("Documents");
        Ok(ProjectDir { path: p })
    }
}

impl std::fmt::Display for ProjectDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
