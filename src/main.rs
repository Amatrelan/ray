mod cli;

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

    let target = cli.target.map(std::path::PathBuf::from);
    let mut brightness = ray::Brightness::try_new(target)?;

    match cli.command {
        cli::Commands::Get => {
            println!("{}", brightness);
            Ok(())
        }
        cli::Commands::Set { percent } => {
            brightness.set_brightness(percent)
        }
        cli::Commands::Increase { percent } => {
            brightness.increase(percent)
        }
        cli::Commands::Decrease { percent } => {
            brightness.decrease(percent)
        }
    }
}
