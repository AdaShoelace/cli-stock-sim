mod login;
mod main_menu;
mod stock_overview;

// Crate imports
pub use self::{login::Login, main_menu::MainMenu, stock_overview::StockOverview};
use super::context::Context;

// Third party imports
use strum::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum ExitReason {
    EnterMainMenu,
    EnterStockOverview,
    Quit,
}

pub trait Activity {
    fn on_create(&mut self, context: Context);
    fn on_draw(&mut self /*context: Context*/);
    fn on_umount(&self) -> Option<&ExitReason>;
    fn on_destroy(&mut self) -> Option<Context>;
}
