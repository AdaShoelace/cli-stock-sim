// Crate imports
use crate::common::{Id, Msg};

// Third party imports
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{
        Alignment, BorderType, Borders, Color, PropPayload, PropValue, TableBuilder, TextModifiers,
        TextSpan,
    },
    tui::{layout::Rect, style::Style, widgets::Paragraph},
    AttrValue, Attribute, Component, Event, Frame, MockComponent, NoUserEvent, Props, State,
    StateValue,
};

use tui_realm_stdlib::List;

#[derive(MockComponent)]
pub struct MainMenu {
    component: List,
}

impl Default for MainMenu {
    fn default() -> Self {
        Self {
            component: List::default()
                .title("Main menu".to_owned(), Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .scroll(true)
                .highlighted_color(Color::Yellow)
                .rewind(true)
                .step(4)
                .rows(
                    TableBuilder::default()
                        .add_col(TextSpan::from("Stock market"))
                        .add_row()
                        .add_col(TextSpan::from("Quit"))
                        .build(),
                )
                .selected_line(0),
        }
    }
}

impl Component<Msg, NoUserEvent> for MainMenu {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Up)),
            Event::Keyboard(KeyEvent {
                code: Key::Down,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Down)),
            Event::Keyboard(KeyEvent {
                code: Key::Up,
                modifiers: KeyModifiers::SHIFT,
            }) => self.perform(Cmd::Scroll(Direction::Up)),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            Event::Keyboard(KeyEvent {
                code: Key::Enter, ..
            }) => {
                /*if let Some(AttrValue::Payload(PropPayload::One(PropValue::Usize(value)))) =
                    self.query(Attribute::Value)
                {
                    match value {
                        0 => return Some(Msg::AppClose),
                        _ => {}
                    }
                }*/
                if let State::One(StateValue::Usize(value)) =
                    self.state()
                {
                    match value {
                        0 => return Some(Msg::Activate(Id::StockOverview)),
                        1 => return Some(Msg::AppClose),
                        _ => {}
                    }
                }
                CmdResult::None
            }
            _ => CmdResult::None,
        };
        Some(Msg::None)
    }
}
