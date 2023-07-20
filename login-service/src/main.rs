mod util;

// Crate imports
use crate::util::init_logging;

// Third party imports
use anyhow::Result;
use log::{debug, warn, info, trace, error};

fn main() -> Result<()> {
    _ = init_logging()?;

    info!("This is info");
    debug!("This is debug");
    warn!("This is warn");
    error!("This is error");
    trace!("This is trace");

    Ok(())
}
