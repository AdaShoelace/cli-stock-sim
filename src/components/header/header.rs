// Crate imports
use crate::common::{Id, Msg};

// Third party imports
use rand::Rng;
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{Alignment, BorderType, Borders, Color, TableBuilder, TextModifiers, TextSpan},
    Component, Event, MockComponent, NoUserEvent, State, StateValue
};

use tui_realm_stdlib::Radio;


#[derive(MockComponent)]
pub struct Header {
    component: Radio,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            component: Radio::default()
                .background(Color::Reset)
                .borders(
                    Borders::default()
                        .modifiers(BorderType::Rounded)
                        .color(Color::LightGreen)
                )
                .foreground(Color::LightGreen)
                .rewind(true)
                .choices(&[
                    "Profile",
                    "Stock Overview",
                    "Main menu"
                ])
                .value(1)
        }
    }
}

impl Component<Msg, NoUserEvent> for Header {
    fn on(&mut self, ev: Event<NoUserEvent>) -> Option<Msg> {
        let cmd_res = match ev {
            Event::Keyboard(KeyEvent {
                code: Key::Left,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Left)),
            Event::Keyboard(KeyEvent {
                code: Key::Right,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Move(Direction::Right)),
            Event::Keyboard(KeyEvent {
                code: Key::Enter,
                modifiers: KeyModifiers::NONE,
            }) => self.perform(Cmd::Submit),
            Event::Keyboard(KeyEvent {
                code: Key::Tab,
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::BlurHeader),
            Event::Keyboard(KeyEvent { code: Key::Esc, .. })
            | Event::Keyboard(KeyEvent {
                code: Key::Backspace,
                ..
            }) => return Some(Msg::ChangeActivity(Id::MainMenu)),
            _ => CmdResult::None,
        };
        
        match cmd_res {
            CmdResult::Submit(State::One(StateValue::Usize(c))) => {
                if c == self.component.states.choices.len() -1 {
                    return Some(Msg::ChangeActivity(Id::MainMenu));
                }
            }
            _ => {}
        }

        Some(Msg::None)
    }
}
