use core::{
    cli::Cli,
    command,
    context::{Context, Initialized, Uninitialized},
    logger,
};

use clap::Parser;
use eyre::Result;
use simple_log::{info, log::warn};
use stopwatch::Stopwatch;

pub mod core;

extern crate reqwest;
extern crate simple_log;
extern crate stopwatch;

fn main() -> Result<()> {
    let sw = Stopwatch::start_new();

    let args = Cli::parse();

    logger::init(args.verbose.log_level())?;

    info!("Parsed CLI arguments: {:#?}", args);

    info!("Creating context...");
    let mut context: Context<Uninitialized> = Context::from_args(&args);

    info!("Checking for missing arguments...");
    if true == context.has_missing_args() {
        warn!("Missing arguments detected. Gathering missing arguments...");
        context.collect_missing_args()?;
    }

    info!("Initializing context...");
    let context: Context<Initialized> = context.init()?;

    info!("Executing command...");
    command::run(&context)?;

    info!("Execution completed in {:?}", sw.elapsed());

    Ok(())
}
