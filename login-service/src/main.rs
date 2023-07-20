mod util;

// Crate imports
use crate::util::init_logging;

// Std imports
use std::env;

// Third party imports
use anyhow::Result;
use log::{debug, error, info, trace, warn};
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<()> {
    _ = init_logging()?;

    let pool = PgPool::connect(&env::var("DATABASE_URL")?).await?;
    _ = get_stock(&pool, 2).await;

    Ok(())
}

async fn get_stock(pool: &PgPool, id: i32) -> Result<()> {
    let res = sqlx::query!(
        r#"
    SELECT * FROM stocks
    WHERE id=($1)
        "#,
        id
    )
    .fetch_one(pool)
    .await?;
    debug!("{:#?}", res);
    Ok(())
}
