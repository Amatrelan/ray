pub mod cli;

pub use color_eyre::Result;

pub static BACKLIGHT_PATH: &str = "/sys/class/backlight";
pub static BRIGHTNESS: &str = "brightness";
pub static MAX_BRIGHTNESS: &str = "max_brightness";
