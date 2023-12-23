//! CLI configuraiton and documentation

use clap::{Parser, Subcommand};

/// Backlight controller what tries to be simple to modify backlight on monitors
#[derive(Parser, Debug)]
#[command(author, version, about, long_about)]
pub struct Cli {
    /// Target where to change brightness
    ///
    /// Defaults to first element in `/sys/class/backlight`. Not sure how stable this is in rust
    pub target:  Option<String>,
    /// Commands available for [`ray`]
    #[command(subcommand)]
    pub command: Commands,

    /// Set logging level verbosity
    #[arg(short, long, default_value_t = 3, action = clap::ArgAction::Count)]
    pub verbosity: u8,
}

/// Available subcommands for [`ray`].
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Get all outputs and their value
    ///
    /// Values are in what kernel tells, not in percentage
    Get,
    /// Set value for target (no limitations)
    Set {
        /// Set values are percentage to simplier to use.
        /// Values in /proc might not be in range 0-100 so
        /// it's simpler to use percentage
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Increase brightness in percentage (cannot be icreased over 100)
    Increase {
        /// Set values are percentage to simplier to use.
        /// Values in /proc might not be in range 0-100 so
        /// it's simpler to use percentage
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
    /// Decrease brightness in percentage (cannot be lowered below 1)
    Decrease {
        /// Set values are percentage to simplier to use.
        /// Values in /proc might not be in range 0-100 so
        /// it's simpler to use percentage
        #[arg(value_parser = clap::value_parser!(u8).range(0..=100))]
        percent: u8,
    },
}

impl Cli {
    /// Perses CLI arguments what are passed by users
    pub fn get() -> Self {
        Self::parse()
    }
}
