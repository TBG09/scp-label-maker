use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;

pub fn setup_logger() -> Result<(), fern::InitError> {
    let colors = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::Green)
        .debug(Color::Blue)
        .trace(Color::BrightBlack);

    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}][{}] {}",
                chrono::Local::now().format("%H:%M:%S"),
                colors.color(record.level()),
                message
            ))
        })
        .level(LevelFilter::Warn)
        .level_for("scp_label_maker", LevelFilter::Info)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}
