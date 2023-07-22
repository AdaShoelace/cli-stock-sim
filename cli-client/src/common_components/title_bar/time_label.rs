// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tui_realm_stdlib::Label;
use tuirealm::{
    props::{Alignment, Color},
    Component, Event, MockComponent
};

#[derive(MockComponent)]
pub struct TimeLabel {
    component: Label,
}

impl Default for TimeLabel {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Right)
                .foreground(Color::Black)
                .background(Color::Green)
        }
    }
}

impl Component<Msg, UserEvent> for TimeLabel {
    fn on(&mut self, _ev: Event<UserEvent>) -> Option<Msg> {
        Some(Msg::None)
    }
}
