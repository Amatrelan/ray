use clap::{Parser, Subcommand};

/// Backlight controller what tries to be simple to modify backlight on monitors
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Target where to change brightness
    ///
    /// Defaults to first element in `/sys/class/backlight`. Not sure how stable this is in rust
    pub target:    Option<String>,
    #[command(subcommand)]
    pub command:   Commands,
    #[arg(short, long, default_value_t = 3, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get all outputs and their value
    ///
    /// Values are in what kernel tells, not in percentage
    Get,
    /// Set value for target
    Set {
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Increase brightness
    Increase {
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Decrease brightness
    Decrease {
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
}

impl Cli {
    pub fn get() -> Self {
        Self::parse()
    }
}
