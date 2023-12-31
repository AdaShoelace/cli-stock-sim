mod activities;
mod activity_manager;
mod common;
mod common_components;
mod context;
mod data_generator;
mod models;
mod util;

// Crate imports
use crate::{activity_manager::{NextActivity, ActivityManager}, common::*};

// Third party imports
use anyhow::Result;

fn main() -> Result<()> {
    util::init_logging()?;
    let mut manager = ActivityManager::new();
    manager.run(NextActivity::Login);
    Ok(())
}
