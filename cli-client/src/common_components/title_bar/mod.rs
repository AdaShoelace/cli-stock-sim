mod balance_label;
mod time_label;

// Crate imports
use crate::common::{Msg, UserEvent, constants::GAME_NAME};
use balance_label::BalanceLabel; 
use time_label::TimeLabel;

// Third party imports
use chrono::{Utc, prelude::*};
use tui_realm_stdlib::{Container, Label};
use tuirealm::{
    props::{Alignment, BorderSides, Borders, Color, Layout},
    tui::layout::{Constraint, Direction},
    Attribute, AttrValue, Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct TitleBar {
    component: Container,
}

impl Default for TitleBar {
    fn default() -> Self {
        Self {
            component: Container::default()
                .borders(Borders::default().sides(BorderSides::NONE))
                .layout(
                    Layout::default()
                        .direction(Direction::Horizontal)
                        .constraints(&[
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                        ])
                        .margin(1),
                )
                .children(vec![
                    Box::new(
                        Label::default()
                            .alignment(Alignment::Left)
                            .foreground(Color::Black)
                            .background(Color::Green)
                            .text(GAME_NAME),
                    ),
                    Box::new(BalanceLabel::default()),
                    Box::new(TimeLabel::default()),
                ]),
        }
    }
}


impl Component<Msg, UserEvent> for TitleBar {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        _ = match ev {
            Event::User(UserEvent::TimeTick(unixtime)) => {
                let time_string = if let Some(naive) = NaiveDateTime::from_timestamp_opt(unixtime, 0) {
                    let datetime: DateTime<Utc> = DateTime::from_utc(naive, Utc);
                    format!("{}", datetime.format("%Y-%m-%d %H:%M:%S"))
                } else {
                    format!("Time Error")
                };
                self.component.children[2].attr(Attribute::Text, AttrValue::String(time_string));
            },
            _ => {}
        };
        Some(Msg::None)
    }
}
