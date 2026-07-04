use std::sync::OnceLock;

use keyring::Entry;
use sqlx::PgPool;

use crate::{
    error_handler::AppError,
    utils::{decrypt, encrypt, prompt},
};

pub static ENCRYPTION_KEY: &str =
    "8vhXf2VkZZjBuvRAW65AH9K8YUjEebfZEMREsv0aTnRhTr0V7ujqFSU28v7wfZbn";
pub static DATABASE_URL: OnceLock<String> = OnceLock::new();
pub static UNIQUE_NOICE: &[u8] = b"T3pCfCPyPUYy";

const SERVICE_NAME: &str = "tuddy";
const SERVICE_KEY: &str = "database";

pub fn load_config() -> Result<bool, AppError> {
    let entry =
        Entry::new(SERVICE_NAME, SERVICE_KEY).map_err(|e| AppError::Config(e.to_string()))?;
    let database_url = match entry.get_password() {
        Ok(val) => decrypt(val.as_str())?,
        Err(_) => return Ok(false),
    };

    DATABASE_URL
        .set(database_url)
        .map_err(|_| AppError::Config("Database URL already loaded".to_string()))?;

    Ok(true)
}

pub async fn set_database_url(database_url: Option<String>) -> Result<(), AppError> {
    let database_url = match database_url {
        Some(val) => val,
        None => prompt(" Enter Database URL: "),
    };

    if database_url.is_empty() {
        return Err(AppError::Config("Database URL cannot be empty".to_string()));
    }

    println!("Validating connection...");
    PgPool::connect(&database_url).await.map_err(|e| {
        AppError::Config(format!("Invalid database URL — connection failed: {}", e))
    })?;

    let encrypted = encrypt(database_url.as_str())?;

    let entry =
        Entry::new(SERVICE_NAME, SERVICE_KEY).map_err(|e| AppError::Config(e.to_string()))?;

    entry
        .set_password(encrypted.as_str())
        .map_err(|e| AppError::Config(e.to_string()))?;

    DATABASE_URL
        .set(database_url)
        .map_err(|_| AppError::Config("Database URL already set".to_string()))?;

    println!("Database configured and connected successfully!");
    Ok(())
}
