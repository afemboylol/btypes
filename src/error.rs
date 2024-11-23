use std::{error::Error, fmt::Display};
/// Possible errors that can occur during BetterString operations
#[derive(Debug, Clone, PartialEq)]
pub enum BStringError {
    EmptyString,
    InvalidOperation(String),
    ConversionError(String),
    EncodingError(String),
    ValidationError(String),
    InvalidUtf8(String),
}

#[derive(Debug, Clone)]
pub enum BBoolError {
    InvalidHeadPos(u8),
    InvalidPos(u8),
    NotFound(String),
    InvalidPattern(String),
    CollectionCapacityReached,
    Other(String),
}

impl Display for BBoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BBoolError::InvalidHeadPos(pos) => format!("Invalid head position: {}", pos),
            BBoolError::InvalidPos(pos) => format!("Invalid position: {}", pos),
            BBoolError::NotFound(item) => format!("Item not found: {}", item),
            BBoolError::InvalidPattern(pat) => format!("Invalid pattern: {}", pat),
            BBoolError::CollectionCapacityReached => "Collection capacity has been reached".to_string(),
            BBoolError::Other(s) => s.to_string(),
        })
    }
}

impl From<anyhow::Error> for BBoolError {
    fn from(error: anyhow::Error) -> Self {
        BBoolError::Other(error.to_string())
    }
}

impl Error for BBoolError {}