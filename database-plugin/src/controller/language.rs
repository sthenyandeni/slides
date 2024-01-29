use serde::{Serialize, ser::SerializeStruct};
use sqlx::{SqlitePool, Error};

pub struct Language {
    key: String,
    language: String
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Language", 2)?;
        state.serialize_field("key", &self.key)?;
        state.serialize_field("language", &self.language)?;
        state.end()
    }
}

pub async fn read_languages(pool: &SqlitePool) -> Result<Vec<Language>, Error> {
    let rows = sqlx::query_as!(Language, "
        SELECT key, language
        FROM language
    ").fetch_all(pool).await?;

    Ok(rows)
}

pub async fn read_language(pool: &SqlitePool, key: String) -> Result<Option<Language>, Error> {
    let result = sqlx::query_as!(Language, "
        SELECT key, language
        FROM language
        WHERE key = $1
    ", key).fetch_optional(pool).await?;

    Ok(result)
}