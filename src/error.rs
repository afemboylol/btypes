use std::{error::Error, fmt::Display};
/// Errors that can occur during `BetterString` operations
///
/// This enum represents various error conditions that may arise when working
/// with `BetterString` operations including validation, encoding, and general
/// string manipulations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BStringError {
    /// Error when attempting to operate on an empty string
    EmptyString,
    /// Error when an operation cannot be completed due to invalid parameters or state
    InvalidOperation(String),
    /// Error when converting between string formats fails
    ConversionError(String),
    /// Error when encoding or decoding operations fail
    EncodingError(String),
    /// Error when string validation fails
    ValidationError(String),
    /// Error when UTF-8 encoding/decoding fails
    InvalidUtf8(String),
}

/// Errors that can occur during `BetterBool` operations
///
/// This enum represents various error conditions that may arise when working
/// with boolean collections, including position validation, pattern matching,
/// and capacity constraints.
#[derive(Debug, Clone)]
pub enum BBoolError {
    /// Error when head position is invalid for fixed-size collections
    InvalidHeadPos(u8),
    /// Error when specified position is invalid for fixed-size collections
    InvalidPos(u8),
    /// Error when head position is invalid for infinite collections
    InvalidHeadPosInf(usize),
    /// Error when specified position is invalid for infinite collections
    InvalidPosInf(usize),
    /// Error when a named boolean value cannot be found
    NotFound(String),
    /// Error when a pattern string is invalid
    InvalidPattern(String),
    /// Error when attempting to exceed collection capacity
    CollectionCapacityReached,
    /// Error when something went wrong internally, such as converting an index from usize to u128.
    InternalError(String, String),
    /// Error for other miscellaneous error conditions
    Other(String),
}

impl Display for BBoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::InvalidHeadPos(pos) => format!("Invalid head position: {pos}"),
                Self::InvalidPos(pos) => format!("Invalid position: {pos}"),
                Self::InvalidHeadPosInf(pos) => format!("Invalid head position: {pos}"),
                Self::InvalidPosInf(pos) => format!("Invalid position: {pos}"),
                Self::NotFound(item) => format!("Item not found: {item}"),
                Self::InvalidPattern(pat) => format!("Invalid pattern: {pat}"),
                Self::CollectionCapacityReached =>
                    "Collection capacity has been reached".to_string(),
                Self::InternalError(t, e) => format!("Internal error of type {t}: {e}"),
                Self::Other(s) => s.to_string(),
            }
        )
    }
}

impl From<anyhow::Error> for BBoolError {
    fn from(error: anyhow::Error) -> Self {
        Self::Other(error.to_string())
    }
}

impl Error for BBoolError {}
