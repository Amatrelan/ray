use std::{fs::read_to_string, io::Write};

use ray::{
    cli::{Cli, Commands},
    Result, BACKLIGHT_PATH, BRIGHTNESS, MAX_BRIGHTNESS,
};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let cli = Cli::get();

    tracing::debug!("{:?}", cli);

    let mut folder = std::fs::read_dir(BACKLIGHT_PATH)?;

    let root = if let Some(target) = cli.target {
        std::path::PathBuf::from(BACKLIGHT_PATH).join(target)
    } else {
        folder.next().unwrap()?.path()
    };

    match cli.command {
        Commands::Get => {
            for each in folder {
                let each = each?;
                let brightness_value = each.path().join("brightness");
                let current = read_to_string(brightness_value)?;
                let current = current.trim();
                println!("{:?}: {}", each.file_name(), current)
            }
        }
        Commands::Set { percent } => {
            let first_folder = root;
            let max_brightness = read_to(&first_folder.join(MAX_BRIGHTNESS))?;
            let new_brightness = new_brightness(percent, max_brightness);
            let brightness_file = first_folder.join(BRIGHTNESS);
            {
                let mut f = std::fs::File::create(brightness_file)?;
                let new_brightness = format!("{new_brightness}");
                f.write_all(new_brightness.as_bytes())?;
            }
        }
        Commands::Increase { percent } => {
            let first_folder = root;
            let max = first_folder.join(MAX_BRIGHTNESS);
            let max_brightness = read_to(&max)?;

            let current = first_folder.join(BRIGHTNESS);
            let current_brightness = read_to(&current)?;

            let amount = new_brightness(percent, max_brightness);

            let new_brightness = if current_brightness + amount < max_brightness {
                amount + current_brightness
            } else {
                max_brightness
            };

            {
                let mut f = std::fs::File::create(current)?;
                let new_brightness = format!("{new_brightness}");
                f.write_all(new_brightness.as_bytes())?;
            }
        }
        Commands::Lower { percent } => {
            let first_folder = root;
            let max = first_folder.join(MAX_BRIGHTNESS);
            let max_brightness = read_to(&max)?;

            let current = first_folder.join(BRIGHTNESS);
            let current_brightness = read_to(&current)?;

            let amount = new_brightness(percent, max_brightness);

            let new_brightness = if current_brightness - amount > 0 {
                current_brightness - amount
            } else {
                1
            };

            {
                let mut f = std::fs::File::create(current)?;
                let new_brightness = format!("{new_brightness}");
                f.write_all(new_brightness.as_bytes())?;
            }
        }
    }
    Ok(())
}

/// Reads path to u32, simple wrapper
fn read_to(value: &std::path::PathBuf) -> Result<u32> {
    let a = read_to_string(value)?;
    let a = a.trim();
    let a: u32 = a.parse()?;
    Ok(a)
}

fn new_brightness(percent: u32, max: u32) -> u32 {
    let percent: f32 = percent as f32 / 100.0;
    let new_brightness = percent * max as f32;
    new_brightness as u32
}
