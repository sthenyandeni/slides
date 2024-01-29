use tauri::{command, State};

use crate::{DbInstance, Result};
use crate::controller::language;

#[command]
pub async fn get_all_langs(
    db: State<'_, DbInstance>
) -> Result<Vec<language::Language>,> {
    let result = language::read_languages(&db.0).await?;
    Ok(result)
}

#[command]
pub async fn get_lang(
    db: State<'_, DbInstance>,
    key: String
) -> Result<Option<language::Language>> {
    let result = language::read_language(&db.0, key).await?;
    Ok(result)
}