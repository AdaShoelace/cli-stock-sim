// Crate imports
use crate::{
    activities::{ExitReason, StockOverview},
    common::{Id, Msg, UserEvent},
    components,
};

// Third party imports
use log::{error, debug};
use tuirealm::{
    props::{Alignment, AttrValue, Attribute},
    Update, Sub, SubEventClause, SubClause
};

impl Update<Msg> for StockOverview {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::BlurHeader => {
                assert!(self.app.active(&Id::StockOverview).is_ok());
                None
            }
            Msg::BlurStockOverview => {
                assert!(self.app.active(&Id::Header).is_ok());
                None
            }
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
                None
            }
            Msg::ChangeActivity(Id::MainMenu) => {
                self.exit_reason = Some(ExitReason::EnterMainMenu);
                None
            }
            Msg::UpdateStockChart(name) => {
                _ = self.tx.send(UserEvent::CurrentStockChanged(name));
                None
            }
            _ => None,
        }
    }
}
