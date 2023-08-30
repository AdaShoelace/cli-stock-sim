mod components;
mod update;
mod view;

use super::{Activity, Context, ExitReason};
use crate::{
    data_generator::{MockDataGenerator, TimeDataGenerator},
    Id, Msg, UserEvent,
};

// std imports
use std::{
    sync::mpsc::{channel, Sender},
    time::Duration,
};

// Third party import
#[allow(unused_imports)]
use log::{debug, error, info};
use tuirealm::{Application, EventListenerCfg, PollStrategy, Update};

pub struct StockOverview {
    app: Application<Id, Msg, UserEvent>,
    exit_reason: Option<ExitReason>,
    context: Option<Context>,
    redraw: bool,
    tx: Sender<UserEvent>,
}

impl StockOverview {
    pub fn new(ticks: Duration) -> Self {
        let (tx, rx) = channel();
        Self {
            app: Application::init(
                EventListenerCfg::default()
                    .default_input_listener(ticks)
                    .poll_timeout(Duration::from_millis(10))
                    .tick_interval(Duration::from_secs(1))
                    .port(
                        Box::new(MockDataGenerator::new((0.0, 0.0), (50.0, 35.0), rx)),
                        Duration::from_millis(400),
                    )
                    .port(
                        Box::new(TimeDataGenerator::default()),
                        Duration::from_secs(5),
                    ),
            ),
            exit_reason: None,
            context: None,
            redraw: true,
            tx,
        }
    }

    fn context_mut(&mut self) -> &mut Context {
        self.context.as_mut().unwrap()
    }
}

impl Activity for StockOverview {
    fn on_create(&mut self, context: Context) {
        info!("Initializing activity: MainMenu");

        self.context = Some(context);

        if let Err(err) = self.context.as_mut().unwrap().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }

        if let Err(err) = self
            .context
            .as_mut()
            .unwrap()
            .terminal()
            .enter_alternate_screen()
        {
            error!("Failed to enter alternate screen: {}", err);
        }

        if let Err(err) = self.context.as_mut().unwrap().terminal().enable_raw_mode() {
            error!("Failed to enable raw mode: {}", err);
        }
        self.init();
        self.app.unlock_subs();

        info!("Activity initialized");
    }

    fn on_draw(&mut self) {
        // Context must be something
        if self.context.is_none() {
            return;
        }
        // Tick
        match self.app.tick(PollStrategy::Once) {
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
        _ = self.app.umount(&Id::StockList);
        _ = self.app.umount(&Id::StockChart);
        self.app.lock_subs();
        // Disable raw mode
        if let Err(err) = self.context_mut().terminal().disable_raw_mode() {
            error!("Failed to disable raw mode: {}", err);
        }
        if let Err(err) = self.context_mut().terminal().clear_screen() {
            error!("Failed to clear screen: {}", err);
        }
        self.context.take()
    }
}
