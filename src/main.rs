use std::str::FromStr;

use color_eyre::eyre::OptionExt;

mod cli;
mod utils;

/// Backlight path where lives controllable backlights
static BACKLIGHT_PATH: &str = "/sys/class/backlight";

fn main() -> color_eyre::Result<()> {
    utils::initialize_panic_handler()?;
    let cli = cli::Cli::get();
    let level = match cli.verbosity {
        1 => tracing::Level::ERROR,
        2 => tracing::Level::WARN,
        3 => tracing::Level::INFO,
        4 => tracing::Level::TRACE,
        _ => tracing::Level::ERROR,
    };

    tracing_subscriber::fmt().with_max_level(level).init();

    match cli.command {
        cli::Commands::Get => {
            let target = match cli.target {
                Some(target) => std::path::PathBuf::from_str(BACKLIGHT_PATH)?.join(target),
                None => match get_first(BACKLIGHT_PATH) {
                    Ok(val) => val,
                    Err(_) => {
                        tracing::error!("No light sources");
                        return Ok(());
                    }
                },
            };

            let brightness = ray::Brightness::try_new(target)?;
            println!("{}", brightness);
            Ok(())
        }
        cli::Commands::Set { percent } => {
            let target = match cli.target {
                Some(target) => std::path::PathBuf::from_str(BACKLIGHT_PATH)?.join(target),
                None => get_first(BACKLIGHT_PATH)?,
            };
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.set_brightness(percent)
        }
        cli::Commands::Increase { percent } => {
            let target = match cli.target {
                Some(target) => std::path::PathBuf::from_str(BACKLIGHT_PATH)?.join(target),
                None => get_first(BACKLIGHT_PATH)?,
            };
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.increase(percent)
        }
        cli::Commands::Decrease { percent } => {
            let target = match cli.target {
                Some(target) => std::path::PathBuf::from_str(BACKLIGHT_PATH)?.join(target),
                None => get_first(BACKLIGHT_PATH)?,
            };
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.decrease(percent)
        }
    }
}

fn get_first(path: &str) -> color_eyre::Result<std::path::PathBuf> {
    let mut folder = std::fs::read_dir(path)?;
    Ok(folder.next().ok_or_eyre("No files in path")??.path())
}
