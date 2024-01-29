use tauri::{command, State};

use crate::{DbInstance, Result};
use crate::controller::song;

#[command]
pub async fn get_book_songs(
    db: State<'_, DbInstance>,
    book_id: i64
) -> Result<Vec<song::SongHeader>> {
    let result = song::read_book_songs(&db.0, book_id).await?;
    Ok(result)
}

#[command]
pub async fn get_song(
    db: State<'_, DbInstance>,
    song_id: i64
) -> Result<song::Song> {
    let result = song::read_song(&db.0, song_id).await?;
    Ok(result)
}