mod cli;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();
    println!("Host: {:#?}", args.host);
    Ok(())
}
