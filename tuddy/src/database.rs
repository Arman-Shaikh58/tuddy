use std::sync::OnceLock;

use chrono::NaiveDate;
use sqlx::{FromRow, PgPool};

use crate::{config::DATABASE_URL, error_handler::AppError};

pub static DB_POOL: OnceLock<PgPool> = OnceLock::new();

/// a single page.
#[derive(Debug, FromRow)]
pub struct JournalEntry {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDate,
}

/// initilize database
pub async fn init_db() -> Result<(), AppError> {
    let database_url = DATABASE_URL.get().ok_or_else(|| {
        AppError::Config("Database URL not configured. Run `tuddy set` first.".to_string())
    })?;

    let pool = PgPool::connect(database_url).await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS entries (
            id          SERIAL PRIMARY KEY,
            title       TEXT NOT NULL,
            body        TEXT NOT NULL,
            created_at  DATE NOT NULL DEFAULT CURRENT_DATE
        )
        "#,
    )
    .execute(&pool)
    .await?;

    DB_POOL
        .set(pool)
        .map_err(|_| AppError::Config("Database pool already initialized".to_string()))?;

    Ok(())
}

// return global pool object
pub fn get_pool() -> Result<&'static PgPool, AppError> {
    DB_POOL
        .get()
        .ok_or_else(|| AppError::Config("Database not initialized".to_string()))
}

pub async fn insert_entry(title: &str, body: &str) -> Result<(), AppError> {
    let pool = get_pool()?;

    sqlx::query("INSERT INTO entries (title, body) VALUES ($1, $2)")
        .bind(title)
        .bind(body)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn get_entries_by_date(date: &NaiveDate) -> Result<Vec<JournalEntry>, AppError> {
    let pool = get_pool()?;

    let rows: Vec<JournalEntry> = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, title, body, created_at FROM entries WHERE created_at = $1 ORDER BY id",
    )
    .bind(date)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_all_entries() -> Result<Vec<JournalEntry>, AppError> {
    let pool = get_pool()?;

    let rows: Vec<JournalEntry> = sqlx::query_as::<_, JournalEntry>(
        "SELECT id, title, body, created_at FROM entries ORDER BY created_at DESC, id DESC",
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn delete_entry(id: i32) -> Result<bool, AppError> {
    let pool = get_pool()?;

    let result = sqlx::query("DELETE FROM entries WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
