pub mod cli;

use std::{fs::read_to_string, io::Write};

pub use color_eyre::Result;

pub static BACKLIGHT_PATH: &str = "/sys/class/backlight";
pub static BRIGHTNESS: &str = "brightness";
pub static MAX_BRIGHTNESS: &str = "max_brightness";

#[tracing::instrument]
pub fn handle_command(cli: cli::Cli) -> Result<()> {
    let cli = cli::Cli::get();
    tracing::debug!("{:?}", cli);

    let root = if let Some(target) = cli.target {
        std::path::PathBuf::from(BACKLIGHT_PATH).join(target)
    } else {
        let mut folder = std::fs::read_dir(BACKLIGHT_PATH)?;
        folder.next().unwrap()?.path()
    };

    match cli.command {
        cli::Commands::Get => {
            let folder = std::fs::read_dir(BACKLIGHT_PATH)?;
            for each in folder {
                let each = each?;
                let current = {
                    let val = read_to_string(each.path().join(BRIGHTNESS))?;
                    let val = val.trim();
                    let val: f32 = val.parse()?;
                    val
                };
                let max = {
                    let val = read_to_string(each.path().join(MAX_BRIGHTNESS))?;
                    let val = val.trim();
                    let val: f32 = val.parse()?;
                    val
                };
                println!("{:?}: {}", each.file_name(), (current / max) * 100.0);
            }
        }
        cli::Commands::Set { percent } => {
            let max_brightness = read_to(&root.join(MAX_BRIGHTNESS))?;
            let new_brightness = value_from_percent(percent, max_brightness);
            let brightness_file = root.join(BRIGHTNESS);
            write_brightness(brightness_file, new_brightness)?;
        }
        cli::Commands::Increase { percent } => {
            let max_brightness = read_to(&root.join(MAX_BRIGHTNESS))?;
            let current = root.join(BRIGHTNESS);
            let current_brightness = read_to(&current)?;
            let amount = value_from_percent(percent, max_brightness);

            let new_brightness = if current_brightness + amount < max_brightness {
                amount + current_brightness
            } else {
                max_brightness
            };

            write_brightness(current, new_brightness)?;
        }
        cli::Commands::Lower { percent } => {
            let max_brightness = read_to(&root.join(MAX_BRIGHTNESS))?;
            let current = root.join(BRIGHTNESS);
            let current_brightness = read_to(&current)?;
            let amount = value_from_percent(percent, max_brightness);

            let mut new_brightness =
                if let Some(new_brightness) = current_brightness.checked_sub(amount) {
                    new_brightness
                } else {
                    1
                };

            if new_brightness < 1 {
                new_brightness = 1;
            }

            write_brightness(current, new_brightness)?;
        }
    }
    Ok(())
}

fn read_to(value: &std::path::PathBuf) -> Result<u32> {
    let a = read_to_string(value)?;
    let a = a.trim();
    let a: u32 = a.parse()?;
    Ok(a)
}

fn value_from_percent(percent: u8, max: u32) -> u32 {
    let percent = {
        if percent > 100 {
            100
        } else {
            percent
        }
    };
    let percent: f32 = percent as f32 / 100.0;
    let new_brightness = percent * max as f32;
    new_brightness as u32
}

#[tracing::instrument]
fn write_brightness(path: std::path::PathBuf, value: u32) -> Result<()> {
    tracing::debug!("Setting brightness");
    let mut f = std::fs::File::create(path)?;
    let new_brightness = format!("{value}");
    f.write_all(new_brightness.as_bytes())?;
    Ok(())
}
