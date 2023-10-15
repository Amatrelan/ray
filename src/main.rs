mod utils;

fn main() -> ray::Result<()> {
    utils::initialize_panic_handler()?;
    let cli = ray::cli::Cli::get();
    let level = match cli.verbosity {
        1 => tracing::Level::ERROR,
        2 => tracing::Level::WARN,
        3 => tracing::Level::INFO,
        4 => tracing::Level::TRACE,
        _ => tracing::Level::ERROR,
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .init();

    return ray::handle_command(cli);
}
