// Crate imports
use crate::common::{Msg, constants};

// Third party imports
use tui_realm_stdlib::Input;
use tuirealm::{
    command::{Cmd, CmdResult,Direction, Position},
    event::{KeyEvent, Key, KeyModifiers, NoUserEvent},
    props::{Alignment, Color, Borders, BorderType, InputType, Style},
    Component, Event, MockComponent, 
};

#[derive(MockComponent)]
pub struct PasswordInput {
    component: Input,
}

impl Default for PasswordInput {
    fn default() -> Self {
        Self {
            component: Input::default()
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightYellow)
                )
                .foreground(Color::LightYellow)
                .input_type(InputType::Password(constants::DOT))
                .title("Password", Alignment::Left)
                .invalid_style(Style::default().fg(Color::Red)),
        }
    }
}

impl Component<Msg, NoUserEvent> for PasswordInput {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let _ = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left, ..
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right, ..
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Home, ..
            }) => self.perform(Cmd::GoTo(Position::Begin)),
            Event::Keyboard(KeyEvent { code: Key::End, .. }) => {
                self.perform(Cmd::GoTo(Position::End))
            }
            Event::Keyboard(KeyEvent {
                code: Key::Delete, ..
            }) => self.perform(Cmd::Cancel),
            Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => self.perform(Cmd::Delete),
            Event::Keyboard(KeyEvent {
                code: Key::Char(ch),
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Type(ch)),
            Event::Keyboard(KeyEvent { code: Key::Tab, .. }) => return Some(Msg::BlurPasswordInput),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => return Some(Msg::AppClose),
            _ => CmdResult::None,
        };

        Some(Msg::None)
    }
}
