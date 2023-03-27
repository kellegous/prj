use crate::ProjectDir;
use std::error::Error;

#[derive(Debug, clap::Args)]
pub struct Args {
    #[clap(long)]
    year: Option<Vec<u32>>,
}

pub fn run(dir: &ProjectDir, args: &Args) -> Result<(), Box<dyn Error>> {
    println!("Search dir={:?}, args={:?}", dir, args);
    Ok(())
}
