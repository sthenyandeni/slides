use serde::{Serialize, ser::SerializeStruct};
use sqlx::{SqlitePool, Error};

pub struct BookTitle {
    language: String,
    title: String
}

impl Serialize for BookTitle {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("BookTitle", 2)?;
        state.serialize_field("language", &self.language)?;
        state.serialize_field("title", &self.title)?;
        state.end()
    }
}

struct BookRecord {
    id: i64
}

pub struct Book {
    id: i64,
    titles: Vec<BookTitle>
}

impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut state = serializer.serialize_struct("Book", 2)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("titles", &self.titles)?;
        state.end()
    }
}

async fn read_book_titles(pool: &SqlitePool, id: i64) -> Result<Vec<BookTitle>, Error> {
    let rows = sqlx::query_as!(BookTitle, "
        SELECT language, title
        FROM book_title
        WHERE book = $1
    ", id).fetch_all(pool).await?;

    Ok(rows)
}

pub async fn read_all_books(pool: &SqlitePool) -> Result<Vec<Book>, Error> {
    let rows = sqlx::query_as!(BookRecord, "
        SELECT id
        FROM book
    ").fetch_all(pool).await?;

    let mut books: Vec<Book> = Vec::new();

    for book in rows.iter() {
        let titles = read_book_titles(pool, book.id).await?;
        books.push(Book {
            id: book.id,
            titles
        });
    }

    Ok(books)
}