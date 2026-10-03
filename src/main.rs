mod args;
mod config;
mod modes;

use anyhow::Result
use clap::Parser;

use args::{Args, Mode};
use config::Config;

fn main() -> Result<()> {
    tracing_susbcriber::fmt()
        .with_target(true)
        .without_time()
        .init();

    let args = Args::parse():
    let config = Config::load(&args)?; 

    match args.mode {
        Mode::Run => modes::run::run(&config),
        Mode::Duo => modes::duo::run(&config),
    }
}
