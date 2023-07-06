// Crate imports
use crate::{activities::{ExitReason, MainMenu}, common::{Id, Msg}};

// Third party imports
use tuirealm::{State, StateValue, Update};

impl Update<Msg> for MainMenu {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
                None
            },
            _ => None
        }
    }
}

