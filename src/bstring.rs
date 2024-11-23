//! BetterString - A more feature-rich string type for Rust
//! 
//! This module provides an enhanced string type with additional operations
//! and utility methods not found in the standard String type.

use std::fmt::{Display, Error};
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign};
use std::str::FromStr;
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};

/// A more convenient alias for BetterString
pub type BStr = BetterString;

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

/// An enhanced string type that provides additional functionality
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BetterString {
    bytes: Vec<u8>,
}

// Implement indexing operations
impl Index<usize> for BetterString {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.bytes[index]
    }
}

// Add string validation methods
impl BetterString {
    /// Validates if the string matches a given pattern
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            match regex::Regex::new(pattern) {
                Ok(re) => re.is_match(s),
                Err(_) => false,
            }
        } else {
            false
        }
    }

    /// Validates if the string is a valid URL
    pub fn is_valid_url(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            url::Url::parse(s).is_ok()
        } else {
            false
        }
    }

    /// Validates if the string is a valid IPv4 address
    pub fn is_valid_ipv4(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.split('.')
                .filter_map(|s| s.parse::<u8>().ok())
                .count() == 4
        } else {
            false
        }
    }
}

// Add encoding conversion methods
impl BetterString {
    /// Converts the string to base64 encoding
    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD.encode(&self.bytes)
    }

    /// Attempts to decode a base64 string
    pub fn from_base64(encoded: &str) -> Result<Self, BStringError> {
        general_purpose::STANDARD.decode(encoded)
            .map_err(|e| BStringError::EncodingError(e.to_string()))
            .map(|bytes| Self { bytes })
    }

    /// Converts the string to URL-safe encoding
    pub fn to_url_encoded(&self) -> String {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            urlencoding::encode(s).into_owned()
        } else {
            String::new()
        }
    }

    /// Decodes a URL-encoded string
    pub fn from_url_encoded(encoded: &str) -> Result<Self, BStringError> {
        urlencoding::decode(encoded)
            .map_err(|e| BStringError::EncodingError(e.to_string()))
            .map(|s| Self::new(s.into_owned()))
    }
}

// Add pattern matching support
impl BetterString {
    /// Finds all matches of a pattern in the string
    pub fn find_all(&self, pattern: &str) -> Vec<(usize, String)> {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let re = regex::Regex::new(pattern).unwrap_or_else(|_| regex::Regex::new(&regex::escape(pattern)).unwrap());
            re.find_iter(s)
                .map(|m| (m.start(), m.as_str().to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Replaces all matches of a pattern with a replacement string
    pub fn replace_all(&self, pattern: &str, replacement: &str) -> Self {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let re = regex::Regex::new(pattern).unwrap_or_else(|_| regex::Regex::new(&regex::escape(pattern)).unwrap());
            Self::new(re.replace_all(s, replacement))
        } else {
            self.clone()
        }
    }
}

// Add additional utility methods
impl BetterString {
    /// Reverses the string
    pub fn reverse(&self) -> Self {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            Self::new(s.chars().rev().collect::<String>())
        } else {
            self.clone()
        }
    }

    /// Counts occurrences of a pattern using regex
    pub fn count_pattern(&self, pattern: &str) -> Result<usize, BStringError> {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            regex::Regex::new(pattern)
                .map_err(|e| BStringError::InvalidOperation(e.to_string()))
                .map(|re| re.find_iter(s).count())
        } else {
            Err(BStringError::InvalidUtf8("Invalid UTF-8 sequence".to_string()))
        }
    }

    /// Checks if the string is a palindrome
    pub fn is_palindrome(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let cleaned = s.chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            cleaned == cleaned.chars().rev().collect::<String>()
        } else {
            false
        }
    }
}

impl BetterString {
    /// Creates a new BetterString from any type that can be converted to a string
    /// 
    /// # Examples
    /// ```
    /// use btypes::bstring::BetterString;
    /// let bstr = BetterString::new("Hello, world!");
    /// ```
    pub fn new<T: ToString>(value: T) -> Self {
        Self {
            bytes: value.to_string().into_bytes(),
        }
    }

    /// Returns the length of the string in bytes
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns true if the string is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns an uppercase version of the string
    pub fn to_uppercase(&self) -> String {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.to_uppercase())
            .unwrap_or_default()
    }

    /// Returns a lowercase version of the string
    pub fn to_lowercase(&self) -> String {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.to_lowercase())
            .unwrap_or_default()
    }

    /// Returns a string with whitespace removed from both ends
    pub fn trim(&self) -> String {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.trim().to_string())
            .unwrap_or_default()
    }

    /// Splits the string by the given delimiter
    /// 
    /// # Arguments
    /// * `delimiter` - The string to split on
    pub fn split(&self, delimiter: &str) -> Vec<String> {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.split(delimiter)
                .map(String::from)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Returns a new string with all occurrences of `from` replaced with `to`
    pub fn replace(&self, from: &str, to: &str) -> String {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.replace(from, to))
            .unwrap_or_default()
    }

    /// Returns true if the string contains the given substring
    pub fn contains(&self, substr: &str) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.contains(substr)
        } else {
            false
        }
    }

    /// Returns true if the string starts with the given prefix
    pub fn starts_with(&self, prefix: &str) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.starts_with(prefix)
        } else {
            false
        }
    }

    /// Returns true if the string ends with the given suffix
    pub fn ends_with(&self, suffix: &str) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.ends_with(suffix)
        } else {
            false
        }
    }

    /// Returns true if the string contains only numeric characters
    pub fn is_numeric(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.chars().all(char::is_numeric)
        } else {
            false
        }
    }

    /// Returns true if the string contains only alphabetic characters
    pub fn is_alphabetic(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.chars().all(char::is_alphabetic)
        } else {
            false
        }
    }

    /// Returns true if the string contains only alphanumeric characters
    pub fn is_alphanumeric(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.chars().all(char::is_alphanumeric)
        } else {
            false
        }
    }

    /// Returns true if the string contains only whitespace
    pub fn is_whitespace(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.chars().all(char::is_whitespace)
        } else {
            false
        }
    }

    /// Performs basic email validation
    /// 
    /// Note: This is a basic implementation and should not be used for
    /// production email validation
    pub fn is_valid_email(&self) -> bool {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let parts: Vec<&str> = s.split('@').collect();
            
            if parts.len() != 2 {
                return false;
            }

            let (local, domain) = (parts[0], parts[1]);
            
            if local.is_empty() || domain.is_empty() || !domain.contains('.') {
                return false;
            }

            !s.chars().any(|c| c.is_whitespace() || c == ',' || c == ';')
        } else {
            false
        }
    }

    /// Returns a substring between the given indices
    /// 
    /// # Arguments
    /// * `start` - The starting index (inclusive)
    /// * `end` - The ending index (exclusive)
    pub fn substring(&self, start: usize, end: usize) -> Result<String, BStringError> {
        if start >= self.len() || end > self.len() || start > end {
            return Err(BStringError::InvalidOperation(
                "Invalid substring indices".to_string(),
            ));
        }

        String::from_utf8(self.bytes[start..end].to_vec())
            .map_err(|e| BStringError::InvalidUtf8(e.to_string()))
    }

    /// Returns the number of words in the string
    pub fn word_count(&self) -> usize {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.split_whitespace().count()
        } else {
            0
        }
    }
}

// Implement basic arithmetic operations
/// Concatenate two strings
impl Add for BetterString {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.bytes.extend_from_slice(&other.bytes);
        self
    }
}

/// Remove a substring
impl SubAssign for BetterString {
    fn sub_assign(&mut self, other: Self) {
        if let (Ok(s1), Ok(s2)) = (std::str::from_utf8(&self.bytes), std::str::from_utf8(&other.bytes)) {
            self.bytes = s1.replace(s2, "").into_bytes();
        }
    }
}

/// Repeat a string n times
impl MulAssign<usize> for BetterString {
    fn mul_assign(&mut self, rhs: usize) {
        let original = self.bytes.clone();
        for _ in 1..rhs {
            self.bytes.extend_from_slice(&original);
        }
    }
}

/// Keep only the first occurence of a substring
impl DivAssign<&str> for BetterString {
    fn div_assign(&mut self, rhs: &str) {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            if let Some(first_pos) = s.find(rhs) {
                let new_s = format!(
                    "{}{}",
                    &s[..first_pos],
                    &s[first_pos + rhs.len()..]
                );
                self.bytes = new_s.into_bytes();
            }
        }
    }
}

/// Concatenate two strings
impl AddAssign for BetterString {
    fn add_assign(&mut self, other: Self) {
        self.bytes.extend_from_slice(&other.bytes);
    }
}

/// Remove a substring
impl Sub for BetterString {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if let (Ok(s1), Ok(s2)) = (std::str::from_utf8(&self.bytes), std::str::from_utf8(&other.bytes)) {
            Self::new(s1.replace(s2, ""))
        } else {
            self
        }
    }
}

/// Repeat a string n times
impl Mul<usize> for BetterString {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            Self::new(s.repeat(rhs))
        } else {
            self
        }
    }
}

/// Count occurences of a substring within a string
impl Div<&str> for BetterString {
    type Output = usize;

    fn div(self, rhs: &str) -> usize {
        if rhs.is_empty() {
            return 0;
        }

        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let mut count = 0;
            let mut start = 0;

            while let Some(pos) = s[start..].find(rhs) {
                count += 1;
                start += pos + rhs.len();
            }

            count
        } else {
            0
        }
    }
}

// Add Iterator support
impl IntoIterator for BetterString {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.into_iter()
    }
}

// Add iterator support for references
impl<'a> IntoIterator for &'a BetterString {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes.iter()
    }
}

// Add byte-related methods
impl BetterString {
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn as_bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    pub fn chars(&self) -> std::str::Chars<'_> {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.chars())
            .unwrap_or_else(|_| "".chars())
    }

    pub fn char_indices(&self) -> std::str::CharIndices<'_> {
        std::str::from_utf8(&self.bytes)
            .map(|s| s.char_indices())
            .unwrap_or_else(|_| "".char_indices())
    }
}

// TODO: Add remaining safe methods, make them standard instead of separate
impl BetterString {
    pub fn safe_split(&self, delimiter: &str) -> Result<Vec<String>, BStringError> {
        if self.is_empty() {
            return Err(BStringError::EmptyString);
        }
        if delimiter.is_empty() {
            return Err(BStringError::InvalidOperation(
                "Delimiter cannot be empty".to_string(),
            ));
        }
        
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => Ok(s.split(delimiter).map(String::from).collect()),
            Err(e) => Err(BStringError::InvalidUtf8(e.to_string())),
        }
    }
}

// Implement Into<String>
impl Into<String> for BetterString {
    fn into(self) -> String {
        String::from_utf8(self.bytes).unwrap_or_default()
    }
}

// Implement Display
impl Display for BetterString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(&self.bytes) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "<invalid utf-8>"),
        }
    }
}

// Add conversion traits
impl From<String> for BetterString {
    fn from(s: String) -> Self {
        Self { bytes: s.into_bytes() }
    }
}

impl From<&str> for BetterString {
    fn from(s: &str) -> Self {
        Self { bytes: s.as_bytes().to_vec() }
    }
}

impl FromStr for BetterString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self { bytes: s.as_bytes().to_vec() })
    }
}
