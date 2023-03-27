use crate::ProjectDir;
use std::error::Error;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long)]
    year: Option<Vec<u32>>,
}

pub fn run(dir: &ProjectDir, args: &Args) -> Result<(), Box<dyn Error>> {
    println!("List dir={:?}, args={:?}", dir, args);

    for dir in dir.year_dirs()? {
        println!("  {}", dir);
    }

    Ok(())
}
