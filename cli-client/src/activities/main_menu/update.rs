// Crate imports
use crate::{activities::{ExitReason, MainMenu}, common::{Msg, Id}};

// Third party imports
use tuirealm::Update;

impl Update<Msg> for MainMenu {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
                None
            }
            Msg::ChangeActivity(Id::StockOverview) => {
                self.exit_reason = Some(ExitReason::EnterStockOverview);
                None
            }
            _ => None
        }
    }
}

