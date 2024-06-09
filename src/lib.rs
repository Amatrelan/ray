//! Backbone of [`ray`]
// #![feature(lint_reasons)] // wait till release. Checks for attributes that allow lints without a reason.
#![allow(clippy::module_name_repetitions)]
// clippy WARN level lints
#![warn(
    missing_docs,
    clippy::cargo,
    clippy::nursery,
    clippy::dbg_macro,
    clippy::unwrap_used,
    clippy::integer_division,
    clippy::large_include_file,
    clippy::map_err_ignore,
    clippy::missing_docs_in_private_items,
    clippy::panic,
    clippy::todo,
    clippy::undocumented_unsafe_blocks,
    clippy::unimplemented,
    clippy::unreachable
)]
// clippy WARN level lints, that can be upgraded to DENY if preferred
#![warn(
    clippy::modulo_arithmetic,
    clippy::assertions_on_result_states,
    clippy::clone_on_ref_ptr,
    clippy::create_dir,
    clippy::default_union_representation,
    clippy::deref_by_slicing,
    clippy::empty_drop,
    clippy::empty_structs_with_brackets,
    clippy::exit,
    clippy::filetype_is_file,
    clippy::float_cmp_const,
    clippy::if_then_some_else_none,
    clippy::indexing_slicing,
    clippy::let_underscore_must_use,
    clippy::lossy_float_literal,
    clippy::pattern_type_mismatch,
    clippy::string_slice,
    clippy::try_err
)]
// clippy DENY level lints, they always have a quick fix that should be preferred
#![deny(
    clippy::wildcard_imports,
    clippy::multiple_inherent_impl,
    clippy::rc_buffer,
    clippy::rc_mutex,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_name_method,
    clippy::self_named_module_files,
    clippy::separated_literal_suffix,
    clippy::shadow_unrelated,
    clippy::str_to_string,
    clippy::string_add,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unseparated_literal_suffix,
    clippy::verbose_file_reads
)]

use std::{fs::read_to_string, io::Write};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Path for current brightness file
static BRIGHTNESS: &str = "brightness";
/// Path for max brightness file
static MAX_BRIGHTNESS: &str = "max_brightness";

/// Wrapper class for brightness folder what is used in linux
pub struct Brightness {
    /// Path of current backlight folder
    root: std::path::PathBuf,
    /// Current backlight value
    current: u32,
    /// Maximum backlight value what is supported
    max: u32,
}

impl Brightness {
    /// Tries to create new [`Brightness`] controller
    pub fn try_new(root: std::path::PathBuf) -> Result<Self> {
        let current: u32 = read_to_string(root.join(BRIGHTNESS))?.trim().parse()?;
        let max: u32 = read_to_string(root.join(MAX_BRIGHTNESS))?.trim().parse()?;

        Ok(Self { root, current, max })
    }

    /// Sets brightness in percentage
    pub fn set_brightness(&mut self, percent: u8) -> Result<()> {
        let previous = self.current;
        let new_brightness = value_from_percent(percent, self.max);
        let brightness_file = self.root.join(BRIGHTNESS);
        write_brightness(brightness_file, new_brightness)?;

        println!(
            "{:?}: {} => {}",
            self.root.file_name(),
            previous,
            new_brightness
        );
        Ok(())
    }

    /// Increase brightness by percentage
    pub fn increase(&mut self, percent: u8) -> Result<()> {
        let amount = value_from_percent(percent, self.max);

        let new_brightness = if self.current + amount < self.max {
            amount + self.current
        } else {
            self.max
        };

        write_brightness(self.root.join(BRIGHTNESS), new_brightness)?;

        Ok(())
    }

    /// Decrease brightness by percentage
    pub fn decrease(&mut self, percent: u8) -> Result<()> {
        let amount = value_from_percent(percent, self.max);

        let mut new_brightness = self
            .current
            .checked_sub(amount)
            .map_or(1, |new_brightness| new_brightness);

        if new_brightness < 1 {
            new_brightness = 1;
        }

        write_brightness(self.root.join(BRIGHTNESS), new_brightness)?;

        Ok(())
    }
}

impl std::fmt::Display for Brightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}: current: {}, max: {}",
            self.root.file_name(),
            self.current,
            self.max
        )
    }
}

/// Converts percentage to value
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

/// Writes new brightness to target path
fn write_brightness(path: std::path::PathBuf, value: u32) -> Result<()> {
    log::debug!("Setting brightness");
    let mut f = std::fs::File::create(path)?;
    let new_brightness = format!("{value}");
    f.write_all(new_brightness.as_bytes())?;
    Ok(())
}
