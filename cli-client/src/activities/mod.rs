mod login;
mod stock_overview;

// Crate imports
pub use self::{login::Login, stock_overview::StockOverview};
use super::context::Context;

// Third party imports
use strum::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum ExitReason {
    EnterStockOverview,
    Quit,
}

pub trait Activity {
    fn on_create(&mut self, context: Context);
    fn on_draw(&mut self /*context: Context*/);
    fn on_umount(&self) -> Option<&ExitReason>;
    fn on_destroy(&mut self) -> Option<Context>;
}
