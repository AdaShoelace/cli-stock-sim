// Crate imports
use super::Counter;
use crate::common::Msg;

use tuirealm::{
    command::{Cmd, CmdResult},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderType, Borders, Color, TextModifiers},
    Component, Event, MockComponent, NoUserEvent, State, StateValue
};

#[derive(MockComponent)]
pub struct LetterCounter {
    component: Counter,
}

impl LetterCounter {
    pub fn new(initial_value: isize) -> Self {
        Self {
            component: Counter::default()
                .alignment(Alignment::Center)
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .color(Color::LightGreen)
                        .modifiers(BorderType::Rounded),
                )
                .foreground(Color::LightGreen)
                .modifiers(TextModifiers::BOLD)
                .value(initial_value)
                .label("Letter counter"),
        }
    }
}

impl Component<Msg, NoUserEvent> for LetterCounter {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) if ch.is_alphabetic() => Cmd::Submit,
            Event::Keyboard(KeyEvent {
                code: Key::Esc,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::AppClose),
            _ => Cmd::None,
        };
        match self.perform(cmd) {
            CmdResult::Changed(State::One(StateValue::Isize(c))) => Some(Msg::ButtonPressed(c)),
            _ => None,
        }
    }
}
