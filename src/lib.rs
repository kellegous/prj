mod list;
mod search;

use clap::Subcommand;
use std::error::Error;
use std::path::{Path, PathBuf};

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
    path: PathBuf,
}

impl ProjectDir {
    pub fn for_user() -> Result<ProjectDir, Box<dyn Error>> {
        let p = home::home_dir()
            .ok_or("Could not find home directory")?
            .join("Documents");
        Ok(ProjectDir { path: p })
    }

    pub fn year_dirs(&self) -> Result<Vec<YearDir>, Box<dyn Error>> {
        // TODO: filter out non-dirs and dirs that aren't a year
        let dirs = std::fs::read_dir(&self.path)?
            .map(|entry| entry.map(|entry| YearDir::new(entry.path())))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(dirs)
    }
}

impl std::fmt::Display for ProjectDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

pub struct YearDir {
    path: PathBuf,
}

impl YearDir {
    pub fn new<P: AsRef<Path>>(dir: P) -> YearDir {
        YearDir {
            path: dir.as_ref().to_path_buf(),
        }
    }
}

impl std::fmt::Display for YearDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}
