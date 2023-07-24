// Crate imports
use crate::{activities::{ExitReason, Login}, common::Msg};

// Third party imports
use tuirealm::Update;

impl Update<Msg> for Login {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
                None
            }
            Msg::Login => {
                // TODO: Validate credentials
                self.exit_reason = Some(ExitReason::EnterStockOverview);
                None
            }
            _ => None
        }
    }
}

