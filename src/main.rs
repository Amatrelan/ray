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

    match cli.command {
        cli::Commands::Get => {
            let brightness = ray::Brightness::try_new(target)?;
            println!("{}", brightness);
            Ok(())
        }
        cli::Commands::Set { percent } => {
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.set_brightness(percent)
        }
        cli::Commands::Increase { percent } => {
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.increase(percent)
        }
        cli::Commands::Decrease { percent } => {
            let mut brightness = ray::Brightness::try_new(target)?;
            brightness.decrease(percent)
        }
    }
}
