use clap_verbosity_flag::Level;
use eyre::{eyre, Result};
use simple_log::LogConfigBuilder;

/// Initialize the logger.
pub fn init(level: Option<Level>) -> Result<()> {
    let config = LogConfigBuilder::builder()
        .level(level.unwrap_or(Level::Error).as_str())
        .output_console()
        .build();

    simple_log::new(config).map_err(|error| eyre!("Cannot compile logger options: {:?}", error))?;

    Ok(())
}
