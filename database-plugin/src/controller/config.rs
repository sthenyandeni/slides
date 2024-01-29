use sqlx::{SqlitePool, Error};

pub struct ConfigEntry {
    key: String,
    pub value: String
}

pub async fn get_config(pool: &SqlitePool, key: String) -> Result<ConfigEntry, Error> {
    let row = sqlx::query_as!(ConfigEntry, "
        SELECT key, value
        FROM config_store
        WHERE key = $1
    ", key).fetch_one(pool).await?;

    Ok(row)
}

pub async fn set_config(pool: &SqlitePool, key: String, value: String) -> Result<(), Error> {
    let config = get_config(pool, key.clone()).await;
    if config.is_err() {
        sqlx::query!("
            INSERT INTO config_store (key, value)
            VALUES ($1, $2)
        ", key, value).execute(pool).await?;

        Ok(())
    }
    else {
        sqlx::query!("
            UPDATE config_store
            SET value = $2
            WHERE key = $1
        ", key, value).execute(pool).await?;

        Ok(())
    }
}