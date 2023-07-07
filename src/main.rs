mod activity_manager;
mod activities;
mod common;
mod context;
mod components;
mod model;
mod util;

// Crate imports
use crate::{activity_manager::{NextActivity, ActivityManager}, common::*};

// Third party imports
use anyhow::Result;

fn main() -> Result<()> {
    util::init_logging()?;
    let mut manager = ActivityManager::new();
    manager.run(NextActivity::MainMenu);
    Ok(())
}
