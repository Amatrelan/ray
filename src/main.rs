use std::str::FromStr;

mod cli;

/// Backlight path where lives controllable backlights
static BACKLIGHT_PATH: &str = "/sys/class/backlight";

fn main() -> ray::Result<()> {
    let cli = cli::Cli::get();
    let level = match cli.verbosity {
        1 => log::LevelFilter::Error,
        2 => log::LevelFilter::Warn,
        3 => log::LevelFilter::Info,
        4 => log::LevelFilter::Trace,
        _ => log::LevelFilter::Error,
    };

    let _ = simplelog::TermLogger::init(
        level,
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    );

    match cli.command {
        cli::Commands::Get => {
            let target: std::path::PathBuf = match cli.target {
                Some(target) => std::path::PathBuf::from_str(BACKLIGHT_PATH)?.join(target),
                None => match get_first(BACKLIGHT_PATH) {
                    Ok(val) => val,
                    Err(_) => {
                        log::error!("No light sources");
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

fn get_first(path: &str) -> ray::Result<std::path::PathBuf> {
    let mut folder = std::fs::read_dir(path)?;
    Ok(folder.next().ok_or("No files found in path")??.path())
}
