mod util;

// Crate imports
use crate::util::init_logging;

// Std imports
use std::env;

// Third party imports
use anyhow::Result;
use sqlx::postgres::PgPool;
use tracing::{debug, info};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging();

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    _ = get_stocks(&pool).await;
    Ok(())
}

async fn get_stocks(pool: &PgPool) -> Result<()> {
    let res = sqlx::query!(
        r#"
            SELECT * FROM stock_view
        "#
    )
    .fetch_all(pool)
    .await?;
    debug!("{:#?}", res);
    Ok(())
}
