// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tuirealm::{
    command::{Cmd, CmdResult, Direction},
    event::{Key, KeyEvent, KeyModifiers},
    props::{BorderType, Borders, Color, },
    Component, Event, MockComponent, State, StateValue
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
                    "Quit"
                ])
                .value(1)
        }
    }
}

impl Component<Msg, UserEvent> for Header {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
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
            Event::Keyboard(KeyEvent {
                code: Key::Char('p'),
                modifiers: KeyModifiers::NONE,
            }) => return Some(Msg::OpenExitPopUp),
            _ => CmdResult::None,
        };
        
        match cmd_res {
            CmdResult::Submit(State::One(StateValue::Usize(c))) => {
                if c == self.component.states.choices.len() -1 {
                    return Some(Msg::OpenExitPopUp);
                }
            }
            _ => {}
        }

        Some(Msg::None)
    }
}
