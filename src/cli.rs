use clap::{Parser, Subcommand};

/// Backlight controller what tries to be simple to modify backlight on monitors
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Target where to change brightness
    ///
    /// Defaults to first element in `/sys/class/backlight`. Not sure how stable this is in rust
    pub target: Option<String>,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get all outputs and their value
    ///
    /// Values are in what kernel tells, not in percentage
    Get,
    /// Set value for target
    Set { percent: u32 },
    /// Increase brightness
    Increase { percent: u32 },
    /// Lower brightness
    Lower { percent: u32 },
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}
