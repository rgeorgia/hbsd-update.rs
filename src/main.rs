mod cli;
pub mod config;

use crate::config::read_config_file;
use clap::Parser;
use cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse().normalize();
    println!("{:#?}", cli);
    // let cfg = read_config_file("/etc/hbsd-update.conf")?;
    let cfg = read_config_file(cli.config_file)?;
    println!("{:#?}", cfg);

    Ok(())
}
