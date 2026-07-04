use crate::{
    config::{ENCRYPTION_KEY, UNIQUE_NOICE},
    error_handler::AppError,
};
use aes_gcm::{
    Aes256Gcm, Nonce,
    aead::{Aead, KeyInit},
};
use chrono::NaiveDate;
use std::io::{self, Write};

// read input from the user
pub fn take_input() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// read multiline input
pub fn take_multiline_input() -> String {
    let mut lines = Vec::new();
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read input");
        let trimmed = line.trim_end_matches('\n').trim_end_matches('\r');
        if trimmed.is_empty() && !lines.is_empty() {
            break;
        }
        lines.push(line);
    }
    lines.concat().trim_end().to_string()
}

pub fn prompt(message: &str) -> String {
    print!("{}", message);
    io::stdout().flush().ok();
    take_input()
}

pub fn prompt_multiline(message: &str) -> String {
    println!("{}", message);
    take_multiline_input()
}

pub fn is_valid_date(date: &str) -> bool {
    NaiveDate::parse_from_str(date, "%Y-%m-%d").is_ok()
}

pub fn parse_date(date: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(date, "%Y-%m-%d")
        .map_err(|_| AppError::Utility(format!("Invalid date format '{}'. Use YYYY-MM-DD.", date)))
}

fn get_cipher() -> Result<Aes256Gcm, AppError> {
    let key_bytes = ENCRYPTION_KEY.as_bytes();
    if key_bytes.len() < 32 {
        return Err(AppError::Utility(
            "Encryption key must be at least 32 bytes".to_string(),
        ));
    }
    let key: [u8; 32] = key_bytes[..32]
        .try_into()
        .map_err(|_| AppError::Utility("Encryption key must be 32 bytes".to_string()))?;

    Aes256Gcm::new_from_slice(&key).map_err(|e| AppError::Utility(e.to_string()))
}

pub fn encrypt(data: &str) -> Result<String, AppError> {
    let cipher = get_cipher()?;

    let nonce = Nonce::try_from(UNIQUE_NOICE.as_ref())
        .map_err(|_| AppError::Utility("failed to create nonce".to_string()))?;

    let encrypted = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(|e| AppError::Utility(e.to_string()))?;

    Ok(hex::encode(encrypted))
}

pub fn decrypt(data: &str) -> Result<String, AppError> {
    let cipher = get_cipher()?;

    let nonce = Nonce::try_from(UNIQUE_NOICE.as_ref())
        .map_err(|_| AppError::Utility("Failed to create nonce".to_string()))?;

    let encrypted = hex::decode(data).map_err(|e| AppError::Utility(e.to_string()))?;

    let decrypted = cipher
        .decrypt(&nonce, encrypted.as_ref())
        .map_err(|e| AppError::Utility(e.to_string()))?;

    String::from_utf8(decrypted).map_err(|e| AppError::Utility(e.to_string()))
}
