use serde::{Serialize, ser::SerializeStruct};
use sqlx::{SqlitePool, Error};

pub struct SongTitle {
    language: String,
    title: String
}

impl Serialize for SongTitle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("SongTitle", 2)?;
        state.serialize_field("language", &self.language)?;
        state.serialize_field("title", &self.title)?;
        state.end()
    }
}

struct SongRecord {
    id: i64,
    sequence: String,
    number: Option<i64>
}

pub struct SongHeader {
    id: i64,
    sequence: String,
    number: Option<i64>,
    titles: Vec<SongTitle>
}

impl Serialize for SongHeader {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("SongHeader", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("titles", &self.titles)?;
        state.end()
    }
}

pub struct SongSection {
    id: i64,
    section: String,
    language: String,
    content: String
}

impl Serialize for SongSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("SongSection", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("section", &self.section)?;
        state.serialize_field("language", &self.language)?;
        state.serialize_field("content", &self.content)?;
        state.end()
    }
}

pub struct Song {
    id: i64,
    sequence: String,
    number: Option<i64>,
    titles: Vec<SongTitle>,
    sections: Vec<SongSection>
}

impl Serialize for Song {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Song", 5)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("sequence", &self.sequence)?;
        state.serialize_field("number", &self.number)?;
        state.serialize_field("titles", &self.titles)?;
        state.serialize_field("sections", &self.sections)?;
        state.end()
    }
}

async fn read_song_titles(pool: &SqlitePool, id: i64) -> Result<Vec<SongTitle>, Error> {
    let rows = sqlx::query_as!(SongTitle, "
        SELECT language, title
        FROM song_title
        WHERE song = $1
    ", id).fetch_all(pool).await?;

    Ok(rows)
}

pub async fn read_book_songs(pool: &SqlitePool, book_id: i64) -> Result<Vec<SongHeader>, Error> {
    let rows = sqlx::query_as!(SongRecord, "
        SELECT id, sequence, number
        FROM song
        WHERE book = $1
    ", book_id).fetch_all(pool).await?;

    let mut songs: Vec<SongHeader> = Vec::new();

    for song in rows.iter() {
        let titles = read_song_titles(pool, song.id).await?;
        songs.push(SongHeader {
            id: song.id,
            sequence: song.sequence.clone(),
            number: song.number,
            titles
        });
    }

    Ok(songs)
}

pub async fn read_song(pool: &SqlitePool, song_id: i64) -> Result<Song, Error> {
    let song_record = sqlx::query_as!(SongRecord, "
        SELECT id, sequence, number
        FROM song
        WHERE id = $1
    ", song_id).fetch_one(pool).await?;

    let titles = read_song_titles(pool, song_record.id).await?;

    let sections = sqlx::query_as!(SongSection, "
        SELECT id, section, language, content
        FROM song_section
        WHERE song = $1
    ", song_id).fetch_all(pool).await?;

    let song = Song {
        id: song_record.id,
        sequence: song_record.sequence.clone(),
        number: song_record.number,
        titles,
        sections
    };

    Ok(song)
}