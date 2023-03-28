mod list;
mod search;

use clap::Subcommand;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

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

#[derive(Debug, Clone)]
pub struct ProjectDir {
    path: Rc<PathBuf>,
}

impl ProjectDir {
    pub fn for_user() -> Result<ProjectDir, Box<dyn Error>> {
        let p = home::home_dir()
            .ok_or("Could not find home directory")?
            .join("Documents");
        Ok(ProjectDir { path: Rc::new(p) })
    }

    pub fn years(&self) -> Result<Vec<Year>, Box<dyn Error>> {
        let dirs = fs::read_dir(self.path.as_ref())?
            .flat_map(|entry| match entry {
                Ok(entry) => Year::from_path(self, entry.path()).map(Ok),
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(dirs)
    }
}

impl std::fmt::Display for ProjectDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path.display())
    }
}

fn parse_year(s: &str) -> Option<u32> {
    match s.parse::<u32>().ok()? {
        y if (1900..=2500).contains(&y) => Some(y),
        _ => None,
    }
}

fn parse_month(s: &str) -> Option<u32> {
    match s.parse::<u32>().ok()? {
        m if (1..=12).contains(&m) => Some(m),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct Year {
    root: ProjectDir,
    year: u32,
}

impl Year {
    pub fn from_path<P: AsRef<Path>>(root: &ProjectDir, dir: P) -> Option<Year> {
        Some(Year {
            root: root.clone(),
            year: parse_year(dir.as_ref().file_name()?.to_str()?)?,
        })
    }

    pub fn months(&self) -> Result<Vec<Month>, Box<dyn Error>> {
        let dirs = fs::read_dir(self.path())?
            .flat_map(|entry| match entry {
                Ok(entry) => Month::from_path(self, entry.path()).map(Ok),
                Err(e) => Some(Err(e)),
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(dirs)
    }

    pub fn year(&self) -> u32 {
        self.year
    }

    pub fn path(&self) -> PathBuf {
        self.root.path.join(self.year.to_string())
    }
}

impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path().display())
    }
}

pub struct Month {
    year: Year,
    month: u32,
}

impl Month {
    fn from_path<P: AsRef<Path>>(year: &Year, dir: P) -> Option<Month> {
        Some(Month {
            year: year.clone(),
            month: parse_month(dir.as_ref().file_name()?.to_str()?)?,
        })
    }

    pub fn month(&self) -> u32 {
        self.month
    }

    pub fn path(&self) -> PathBuf {
        self.year.path().join(self.month.to_string())
    }
}

impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path().display())
    }
}
