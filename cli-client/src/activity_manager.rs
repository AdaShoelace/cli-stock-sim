// Crate imports
use crate::{activities::{Activity, ExitReason, StockOverview, Login}, context::Context};

// Std imports
use std::time::Duration;

// Third party imports
#[allow(unused_imports)]
use log::{debug, error, info};

pub enum NextActivity {
    Login,
    StockOverview
}

pub struct ActivityManager {
    context: Option<Context>,
}

impl ActivityManager {
    pub fn new() -> Self {
        Self { context: Some(Context::new()) }
    }

    pub fn run(&mut self, initial_activity: NextActivity) {
        let mut current_activity = Some(initial_activity);
        loop {
            current_activity = match current_activity {
                Some(activity) => match activity {
                    NextActivity::Login => self.run_login(),
                    NextActivity::StockOverview => self.run_stock_overview()
                },
                None => break
            }
        }
        drop(self.context.take())
    }
    fn run_login(&mut self) -> Option<NextActivity> {
        let mut activity = Login::new(Duration::from_millis(20));
        
        let result: Option<NextActivity>;
        
        let ctx = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to initialize MainMenu activity: context is None");
                return None
            }
        };

        activity.on_create(ctx);
        loop {
            activity.on_draw();

            if let Some(exit_reason) = activity.on_umount() {
                match exit_reason {
                    ExitReason::Quit => {
                        info!("Login activity terminated due to {}", exit_reason);
                        result = None;
                        break;
                    }
                    ExitReason::EnterStockOverview => {
                        info!("Leaving Login for StockOverview due to: {}", exit_reason);
                        result = Some(NextActivity::StockOverview);
                        break;
                    }
                }
            }
        }
        self.context = activity.on_destroy();
        info!("Login acitivity destroyed");
        result

    }

    fn run_stock_overview(&mut self) -> Option<NextActivity> {
        let mut activity = StockOverview::new(Duration::from_millis(20));
        
        let result: Option<NextActivity>;
        
        let ctx = match self.context.take() {
            Some(ctx) => ctx,
            None => {
                error!("Failed to initialize StockOverview activity: context is None");
                return None
            }
        };

        activity.on_create(ctx);
        loop {
            activity.on_draw();

            if let Some(exit_reason) = activity.on_umount() {
                match exit_reason {
                    ExitReason::Quit => {
                        info!("StockOverview activity terminated due to {}", exit_reason);
                        result = None;
                        break;
                    }
                    _ => {}
                }
            }
        }
        self.context = activity.on_destroy();
        info!("StockOverview acitivity destroyed");
        result
    }
}
