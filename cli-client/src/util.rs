use anyhow::Result;
use tuirealm::{
    props::{Alignment, Borders, Color, Style},
    tui::widgets::Block,
};

pub fn get_block<'a>(
    props: Borders,
    title: Option<(String, Alignment)>,
    focus: bool,
    inactive_style: Option<Style>,
) -> Block<'a> {
    let title = title.unwrap_or((String::default(), Alignment::Left));
    Block::default()
        .borders(props.sides)
        .border_style(match focus {
            true => props.style(),
            false => {
                inactive_style.unwrap_or_else(|| Style::default().fg(Color::Reset).bg(Color::Reset))
            }
        })
        .border_type(props.modifiers)
        .title(title.0)
        .title_alignment(title.1)
}

pub fn init_logging() -> Result<()> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                humantime::format_rfc3339(std::time::SystemTime::now()),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(fern::log_file("/tmp/cli-stock.log")?)
        .apply()?;
    Ok(())
}
