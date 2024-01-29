use tauri::{command, State};

use crate::{DbInstance, Result};
use crate::controller::book;

#[command]
pub async fn get_all_books(
    db: State<'_, DbInstance>,
) -> Result<Vec<book::Book>> {
    let result = book::read_all_books(&db.0).await?;
    Ok(result)
}