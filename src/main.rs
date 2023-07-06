mod activity_manager;
mod activities;
mod common;
mod context;
mod components;
mod model;
mod util;

// Crate imports
use crate::{activity_manager::{NextActivity, ActivityManager}, common::*, model::Model};

// Third party imports
use anyhow::Result;
use tuirealm::{AttrValue, Attribute, PollStrategy, Update};

fn main() -> Result<()> {
    util::init_logging()?;
    /*let mut model = Model::default();

    let _ = model.terminal.enter_alternate_screen();
    let _ = model.terminal.enable_raw_mode();

    while !model.quit {
        match model.app.tick(PollStrategy::Once) {
            Err(err) => {
                assert!(model
                    .app
                    .attr(
                        &Id::Label,
                        Attribute::Text,
                        AttrValue::String(format!("Application error: {}", err)),
                    )
                    .is_ok());
            }
            Ok(messages) if messages.len() > 0 => {
                model.redraw = true;
                for msg in messages.into_iter() {
                    let mut msg = Some(msg);
                    while msg.is_some() {
                        msg = model.update(msg);
                    }
                }
            }
            _ => {}
        }

        if model.redraw {
            model.view();
            model.redraw = false;
        }
    }

    let _ = model.terminal.leave_alternate_screen();
    let _ = model.terminal.disable_raw_mode();
    let _ = model.terminal.clear_screen();
    */

    let mut manager = ActivityManager::new();
    manager.run(NextActivity::MainMenu);
    Ok(())
}
