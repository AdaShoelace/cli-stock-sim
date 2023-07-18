// Crate imports
use crate::common::{Msg, UserEvent};

// Third party imports
use tui_realm_stdlib::Label;
use tuirealm::{
    props::{Alignment, Color},
    Component, Event, MockComponent,
};

#[derive(MockComponent)]
pub struct BalanceLabel {
    component: Label,
}

impl Default for BalanceLabel {
    fn default() -> Self {
        Self {
            component: Label::default()
                .alignment(Alignment::Center)
                .foreground(Color::Black)
                .background(Color::Green)
                .text("0$")
        }
    }
}

impl Component<Msg, UserEvent> for BalanceLabel {
    fn on(&mut self, _ev: Event<UserEvent>) -> Option<Msg> {
        Some(Msg::None)
    }
}
