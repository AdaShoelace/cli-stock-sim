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

pub mod popup {
    use tuirealm::tui::layout::{Constraint, Direction, Layout, Rect};

    /// Size type for UI renders
    #[allow(dead_code)]
    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
    pub enum Size {
        Percentage(u16),
        Unit(u16),
    }

    /// Ui popup dialog (w x h)
    pub struct Popup(pub Size, pub Size);

    impl Popup {
        /// Draw popup in provided area
        pub fn draw_in(&self, parent: Rect) -> Rect {
            let new_area = Layout::default()
                .direction(Direction::Vertical)
                .constraints(self.height(&parent).as_ref())
                .split(parent);
            Layout::default()
                .direction(Direction::Horizontal)
                .constraints(self.width(&parent).as_ref())
                .split(new_area[1])[1]
        }

        fn height(&self, parent: &Rect) -> [Constraint; 3] {
            Self::constraints(parent.height, self.1)
        }

        fn width(&self, parent: &Rect) -> [Constraint; 3] {
            Self::constraints(parent.width, self.0)
        }

        fn constraints(area_size: u16, popup_size: Size) -> [Constraint; 3] {
            match popup_size {
                Size::Percentage(popup_size) => [
                    Constraint::Percentage((100 - popup_size) / 2),
                    Constraint::Percentage(popup_size),
                    Constraint::Percentage((100 - popup_size) / 2),
                ],
                Size::Unit(popup_size) => {
                    let margin = (area_size - popup_size) / 2;
                    [
                        Constraint::Length(margin),
                        Constraint::Length(popup_size),
                        Constraint::Length(margin),
                    ]
                }
            }
        }
    }
}
