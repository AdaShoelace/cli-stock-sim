// Crate imports
use crate::{activities::{ExitReason, StockOverview}, common::{Id, Msg}};

// Third party imports
use tuirealm::Update;


impl Update<Msg> for StockOverview {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
                None
            }
            Msg::ChangeActivity(Id::MainMenu) => {
                self.exit_reason = Some(ExitReason::EnterMainMenu);
                None
            }
            _ => None
        }
    }
}

