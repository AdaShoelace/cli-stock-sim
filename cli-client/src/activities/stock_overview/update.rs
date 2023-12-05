// Crate imports
use crate::{
    activities::{ExitReason, StockOverview},
    common::{Id, Msg, UserEvent},
};
use super::components::{BuyPopUp, ExitPopUp};

// Third party imports
use tuirealm::Update;

impl Update<Msg> for StockOverview {
    fn update(&mut self, msg: Option<Msg>) -> Option<Msg> {
        self.redraw = true;
        match msg.unwrap_or(Msg::None) {
            Msg::BlurHeader => {
                assert!(self.app.active(&Id::StockList).is_ok());
                None
            }
            Msg::BlurStockList => {
                assert!(self.app.active(&Id::Header).is_ok());
                None
            }
            Msg::OpenExitPopUp => {
                self.app.lock_subs();
                assert!(self.app.mount(Id::ExitPopUp, Box::new(ExitPopUp::default()), vec![]).is_ok());
                None
            }
            Msg::CloseExitPopUp => {
                self.app.unlock_subs();
                assert!(self.app.umount(&Id::ExitPopUp).is_ok());
                None
            }
            Msg::OpenBuyPopUp => {
                self.app.lock_subs();
                assert!(self.app.mount(Id::BuyPopUp, Box::new(BuyPopUp::default()), vec![]).is_ok());
                None
            }
            Msg::CloseBuyPopUp => {
                self.app.unlock_subs();
                assert!(self.app.umount(&Id::BuyPopUp).is_ok());
                None
            }
            Msg::AppClose => {
                self.exit_reason = Some(ExitReason::Quit);
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
