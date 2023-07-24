// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use log::debug;
use tui_realm_stdlib::Phantom;
use tuirealm::{
    event::{Key, KeyEvent},
    Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct GlobalListener {
    component: Phantom,
}

impl Default for GlobalListener {
    fn default() -> Self {
        Self {
            component: Phantom::default(),
        }
    }
}

impl Component<Msg, UserEvent> for GlobalListener {
    fn on(&mut self, ev: Event<UserEvent>) -> Option<Msg> {
        match ev {
            Event::Keyboard(KeyEvent { code: Key::Esc, .. }) => {
                debug!("{:#?}", ev);
                Some(Msg::OpenExitPopUp)
            }
            _ => Some(Msg::None),
        }
    }
}
