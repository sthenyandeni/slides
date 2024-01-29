use tauri::{command, State};

use crate::{DbInstance, Result};
use crate::controller::config;

#[command]
pub async fn get_config(
    db: State<'_, DbInstance>,
    key: String
) -> Result<String> {
    let config = config::get_config(&db.0, key).await?;
    Ok(config.value)
}

#[command]
pub async fn set_config(
    db: State<'_, DbInstance>,
    key: String,
    value: String
) -> Result<()> {
    config::set_config(&db.0, key, value).await?;
    Ok(())
}