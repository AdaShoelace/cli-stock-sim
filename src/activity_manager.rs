// Crate imports
use crate::{activities::{Activity, ExitReason, MainMenu}, context::Context};

// Std imports
use std::time::Duration;

// Third party imports
#[allow(unused_imports)]
use log::{debug, error, info};

pub enum NextActivity {
    MainMenu,
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
                    NextActivity::MainMenu => self.run_main_menu(),
                    NextActivity::StockOverview => self.run_stock_overview()
                },
                None => break
            }
        }
        drop(self.context.take())
    }

    fn run_main_menu(&mut self) -> Option<NextActivity> {
        let mut activity = MainMenu::new(Duration::from_millis(20));
        
        let mut result: Option<NextActivity>;
        
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
                        info!("MainMenu activity terminated due to {}", exit_reason);
                        result = None;
                        break;
                    }
                    _ => {}
                }
            }
        }
        self.context = activity.on_destroy();
        info!("MainMenu acitivity destroyed");
        result
    }

    fn run_stock_overview(&mut self) -> Option<NextActivity> {
        unimplemented!()
    }
}
