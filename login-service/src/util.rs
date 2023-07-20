// Third party imports
use chrono::Utc;
use anyhow::Result;
use fern::{
    colors::{Color, ColoredLevelConfig},
    Dispatch,
};

pub fn init_logging() -> Result<()> {
    let colors_line = ColoredLevelConfig::new()
        .error(Color::Red)
        .warn(Color::Yellow)
        .info(Color::White)
        .debug(Color::Cyan)
        .trace(Color::BrightBlack);

    let colors_level = colors_line.info(Color::Green);

    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{date} {level} {target}] {message}",
                date = Utc::now().format("%Y-%m-%d %H:%M:%S"),
                target = record.target(),
                level = colors_level.color(record.level()),
                message = message,
            ));
        })
        .level(log::LevelFilter::Trace)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("/tmp/login-service.log")?)
        .apply()?;
    Ok(())
}
