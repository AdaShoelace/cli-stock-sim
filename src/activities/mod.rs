mod main_menu;

// Crate imports
use super::context::Context;
pub use main_menu::MainMenu;

// Third party imports
use strum::{Display, EnumString};

#[derive(EnumString, Display)]
pub enum ExitReason {
    EnterMainMenu,
    EnterStockOverview,
    Quit
}

pub trait Activity {
    fn on_create(&mut self, context: Context);
    fn on_draw(&mut self, /*context: Context*/ );
    fn on_umount(&self) -> Option<&ExitReason>;
    fn on_destroy(&mut self) -> Option<Context>;
}
