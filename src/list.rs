use crate::ProjectDir;
use std::error::Error;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long, short)]
    year: Option<Vec<u32>>,
}

pub fn run(dir: &ProjectDir, args: &Args) -> Result<(), Box<dyn Error>> {
    println!("List dir={:?}, args={:?}", dir, args);

    for year in dir.years()? {
        for month in year.months()? {
            for project in month.projects()? {
                println!("{}", project.path().display());
            }
        }
    }

    Ok(())
}
