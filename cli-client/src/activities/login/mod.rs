mod components;
mod update;
mod view;

// Crate imports
use super::{Activity, Context, ExitReason};
use crate::{Id, Msg};

// std imports
use std::time::Duration;

// Third party import
#[allow(unused_imports)]
use log::{debug, error, info};
use tuirealm::{Application, EventListenerCfg, NoUserEvent, PollStrategy, Update};

pub struct Login {
    app: Application<Id, Msg, NoUserEvent>,
    exit_reason: Option<ExitReason>,
    context: Option<Context>,
    redraw: bool,
}

impl Login {
    pub fn new(ticks: Duration) -> Self {
        Self {
            app: Application::init(
                EventListenerCfg::default()
                    .default_input_listener(ticks)
                    .poll_timeout(Duration::from_millis(10))
                    .tick_interval(Duration::from_secs(1)),
            ),
            exit_reason: None,
            context: None,
            redraw: true,
        }
    }

    fn context_mut(&mut self) -> &mut Context {
        self.context.as_mut().unwrap()
    }
}

impl Activity for Login {
    fn on_create(&mut self, context: Context) {
        info!("Initializing activity: Login");

        self.context = Some(context);

        if let Err(err) = self.context.as_mut().unwrap().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
    
        if let Err(err) = self.context.as_mut().unwrap().terminal().enter_alternate_screen() {
            error!("Failed to enter alternate screen: {}", err);
        }

        if let Err(err) = self.context.as_mut().unwrap().terminal().enable_raw_mode() {
            error!("Failed to enable raw mode: {}", err);
        }
        
        self.init();

        info!("Activity initialized");
    }

    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Tick
        match self.app.tick(PollStrategy::UpTo(3)) {
            Ok(messages) => {
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = self.update(msg);
                    }
                }
            }
            Err(err) => {
                panic!("{}", err)
                //self.mount_error(format!("Application error: {err}"));
            }
        }
        // View
        if self.redraw {
            self.view();
        }
    }

    fn on_umount(&self) -> Option<&ExitReason> {
        self.exit_reason.as_ref()
    }

    fn on_destroy(&mut self) -> Option<Context> {
        // Disable raw mode
        /*if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }*/
        if let Err(err) = self.context_mut().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        self.context.take()
    }
}
