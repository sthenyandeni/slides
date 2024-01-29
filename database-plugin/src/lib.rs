
use serde::{ser::Serializer, Serialize};
use sqlx::SqlitePool;
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

// use std::{collections::HashMap, sync::Mutex};

mod controller;

mod api;
use api as commands;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Sql(#[from] sqlx::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

struct DbInstance(SqlitePool);

#[derive(Default)]
struct MyState {
    
}


/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("database")
        .invoke_handler(tauri::generate_handler![
            commands::config::get_config,
            commands::config::set_config,
            commands::language::get_all_langs, 
            commands::language::get_lang, 
            commands::book::get_all_books,
            commands::song::get_book_songs,
            commands::song::get_song
        ])
        .setup(|app| {
            let _ = tauri::async_runtime::block_on(async {
                let pool = SqlitePool::connect("sqlite:/home/sthenyandeni/Development/slides/database.db").await.expect("Noooo");
                let db_instance = DbInstance(pool);
                app.manage(db_instance);
                app.manage(MyState::default());

                Ok::<(), Error>(())
            });
            Ok(())
        })
        .build()
}
