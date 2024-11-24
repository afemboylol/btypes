use crate::error::BStringError;
use base64::engine::general_purpose;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Error};
use std::hash::Hash;
use std::ops::{
    Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, Mul, MulAssign, Sub, SubAssign,
};
use std::str::FromStr;

// This is really a trash type ngl. For a type in "BetterTypes" it's not good enough.

/// A more convenient alias for `BetterString`
pub type BStr = BetterString;

/// An enhanced string type that provides additional functionality
#[derive(Debug, Clone, Eq)]
pub struct BetterString {
    bytes: Vec<u8>,
}

impl Hash for BetterString
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.bytes);
    }
}

impl PartialEq<&str> for BetterString
{
    fn eq(&self, other: &&str) -> bool {
        &self.as_str() == other
    }
}

impl PartialEq<Self> for BetterString
{
    fn eq(&self, other: &Self) -> bool {
        self.bytes == other.bytes
    }
}

impl Serialize for BetterString {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // Serialize the bytes directly
        serializer.serialize_bytes(&self.bytes)
    }
}

impl<'de> Deserialize<'de> for BetterString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // We'll use a visitor to deserialize the bytes
        struct BetterStringVisitor;

        impl<'de> serde::de::Visitor<'de> for BetterStringVisitor {
            type Value = BetterString;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a byte array or string")
            }

            // Handle byte array input
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BetterString { bytes: v.to_vec() })
            }

            // Handle string input
            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BetterString {
                    bytes: v.as_bytes().to_vec(),
                })
            }

            // Handle borrowed string input
            fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BetterString {
                    bytes: v.as_bytes().to_vec(),
                })
            }

            // Handle string input
            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(BetterString {
                    bytes: v.into_bytes(),
                })
            }
        }

        // Use the appropriate deserializer based on the input format
        deserializer.deserialize_bytes(BetterStringVisitor)
    }
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
    #[must_use]
    pub fn matches_pattern(&self, pattern: &str) -> bool {
        std::str::from_utf8(&self.bytes)
            .is_ok_and(|s| regex::Regex::new(pattern).is_ok_and(|re| re.is_match(s)))
    }

    /// Validates if the string is a valid URL
    #[must_use]
    pub fn is_valid_url(&self) -> bool {
        // First ensure the bytes are valid UTF-8
        let url_str = match std::str::from_utf8(&self.bytes) {
            Ok(s) => s.trim(),
            Err(_) => return false,
        };

        // Check if string is empty or too long (arbitrary max length of 2083, same as IE)
        if url_str.is_empty() || url_str.len() > 2083 {
            return false;
        }

        // Split URL into scheme and rest
        let parts: Vec<&str> = url_str.splitn(2, "://").collect();
        if parts.len() != 2 {
            return false;
        }

        let scheme = parts[0].to_lowercase();
        let remainder = parts[1];

        // Validate scheme
        if scheme.is_empty()
            || !scheme
                .chars()
                .all(|c| c.is_ascii_alphabetic() || c == '+' || c == '.' || c == '-')
        {
            return false;
        }

        // Must have at least one character after scheme
        if remainder.is_empty() {
            return false;
        }

        // Split remainder into authority and path
        let mut authority = remainder;
        let mut path = "";
        if let Some(idx) = remainder.find('/') {
            authority = &remainder[..idx];
            path = &remainder[idx..];
        }

        // Validate authority (host)
        if authority.is_empty() {
            return false;
        }

        // Split authority into userinfo, host, and port
        let mut host = authority;
        if let Some(idx) = authority.rfind('@') {
            // Has userinfo
            host = &authority[idx + 1..];
        }

        // Handle port if present
        if let Some(idx) = host.rfind(':') {
            let port = &host[idx + 1..];
            // Validate port
            if !port.chars().all(|c| c.is_ascii_digit()) || port.len() > 5 {
                return false;
            }
            host = &host[..idx];
        }

        // Validate host
        if host.is_empty() {
            return false;
        }

        // Check if host is IPv4
        if host.chars().all(|c| c.is_ascii_digit() || c == '.') {
            let octets: Vec<&str> = host.split('.').collect();
            if octets.len() != 4 {
                return false;
            }
            for octet in octets {
                if octet.is_empty() || octet.len() > 3 {
                    return false;
                }
                match octet.parse::<u8>() {
                    Ok(_) => continue,
                    Err(_) => return false,
                }
            }
        } else {
            // Validate hostname
            let labels: Vec<&str> = host.split('.').collect();
            if labels.len() < 2 {
                return false;
            }
            for label in labels {
                if label.is_empty() || label.len() > 63 {
                    return false;
                }
                if !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-') {
                    return false;
                }
                if label.starts_with('-') || label.ends_with('-') {
                    return false;
                }
            }
        }

        // Basic path validation
        if !path.is_empty()
            && !path
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "/-._~!$&'()*+,;=:@%".contains(c))
        {
            return false;
        }

        true
    }

    /// Validates if the string is a valid IPv4 address
    #[must_use]
    pub fn is_valid_ipv4(&self) -> bool {
        std::str::from_utf8(&self.bytes)
            .is_ok_and(|s| s.split('.').filter_map(|s| s.parse::<u8>().ok()).count() == 4)
    }
}

// Add encoding conversion methods
impl BetterString {
    /// Converts the string to base64 encoding
    #[must_use]
    pub fn to_base64(&self) -> Self {
        Self::new(general_purpose::STANDARD.encode(&self.bytes))
    }

    /// Attempts to decode a base64 string
    ///
    /// # Errors
    ///
    /// Returns a `BStringError::EncodingError` if the input string is not valid base64
    pub fn from_base64(encoded: &Self) -> Result<Self, BStringError> {
        general_purpose::STANDARD
            .decode(encoded)
            .map_err(|e| BStringError::EncodingError(e.to_string()))
            .map(|bytes| Self { bytes })
    }

    /// Converts the string to URL-safe encoding
    #[allow(clippy::option_if_let_else)]
    #[must_use]
    pub fn to_url_encoded(&self) -> Self {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            Self::new(urlencoding::encode(s))
        } else {
            Self::empty()
        }
    }

    /// Decodes a URL-encoded string
    ///
    /// # Errors
    ///
    /// Returns `BStringError::EncodingError` if the input string contains invalid URL-encoded characters
    /// or malformed percent-encoding sequences.
    pub fn from_url_encoded(encoded: &Self) -> Result<Self, BStringError> {
        urlencoding::decode(encoded)
            .map_err(|e| BStringError::EncodingError(e.to_string()))
            .map(|s| Self::new(s.to_string()))
    }

    /// Returns an empty `BetterString`.
    #[must_use]
    pub fn empty() -> Self {
        Self::new("")
    }
}

// Add pattern matching support
impl BetterString {
    /// Finds all matches of a pattern in the string
    ///
    /// # Panics
    ///
    /// Panics if creating a new `RegEx` from the escaped pattern fails.
    #[allow(clippy::option_if_let_else)]
    #[must_use]
    pub fn find_all(&self, pattern: &str) -> Vec<(usize, Self)> {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let re = regex::Regex::new(pattern)
                .unwrap_or_else(|_| regex::Regex::new(&regex::escape(pattern)).unwrap());
            re.find_iter(s)
                .map(|m| (m.start(), Self::from(m.as_str().to_string())))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Replaces all matches of a pattern with a replacement string
    ///
    /// # Panics
    ///
    /// Panics if creating a new `RegEx` from the escaped pattern fails.
    #[allow(clippy::option_if_let_else)]
    #[must_use]
    pub fn replace_all(&self, pattern: &str, replacement: &str) -> Self {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            let re = regex::Regex::new(pattern)
                .unwrap_or_else(|_| regex::Regex::new(&regex::escape(pattern)).unwrap());
            Self::new(re.replace_all(s, replacement))
        } else {
            self.clone()
        }
    }
}

// Add additional utility methods
impl BetterString {
    /// Reverses the string
    #[must_use]
    pub fn reverse(&self) -> Self {
        std::str::from_utf8(&self.bytes).map_or_else(
            |_| self.clone(),
            |s| Self::new(s.chars().rev().collect::<String>()),
        )
    }

    /// Counts occurrences of a pattern using regex
    ///
    /// # Errors
    ///
    /// Returns `BStringError::InvalidOperation` if creating a new `RegEx` from the pattern fails.
    pub fn count_pattern(&self, pattern: &str) -> Result<usize, BStringError> {
        std::str::from_utf8(&self.bytes).map_or_else(
            |_| {
                Err(BStringError::InvalidUtf8(
                    "Invalid UTF-8 sequence".to_string(),
                ))
            },
            |s| {
                regex::Regex::new(pattern)
                    .map_err(|e| BStringError::InvalidOperation(e.to_string()))
                    .map(|re| re.find_iter(s).count())
            },
        )
    }

    /// Checks if the string is a palindrome
    #[must_use]
    pub fn is_palindrome(&self) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| {
            let cleaned = s
                .chars()
                .filter(|c| c.is_alphanumeric())
                .collect::<String>()
                .to_lowercase();
            cleaned == cleaned.chars().rev().collect::<String>()
        })
    }
}

impl BetterString {
    /// Returns self as an iterator
    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        <&Self as IntoIterator>::into_iter(self)
    }
}

impl BetterString {
    /// Creates a new `BetterString` from any type that can be converted to a string
    ///
    /// # Examples
    /// ```
    /// use btypes::bstring::BetterString;
    /// let bstr = BetterString::new("Hello, world!");
    /// ```
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<T: ToString>(value: T) -> Self {
        Self {
            bytes: value.to_string().into_bytes(),
        }
    }

    /// Returns the length of the string in bytes
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Returns true if the string is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Returns an uppercase version of the string
    #[must_use]
    pub fn to_uppercase(&self) -> Self {
        Self::new(
            std::str::from_utf8(&self.bytes)
                .map(str::to_uppercase)
                .unwrap_or_default(),
        )
    }

    /// Returns a lowercase version of the string
    #[must_use]
    pub fn to_lowercase(&self) -> Self {
        Self::new(
            std::str::from_utf8(&self.bytes)
                .map(str::to_lowercase)
                .unwrap_or_default(),
        )
    }

    /// Returns a string with whitespace removed from both ends
    #[must_use]
    pub fn trim(&self) -> Self {
        Self::new(
            std::str::from_utf8(&self.bytes)
                .map(|s| s.trim().to_string())
                .unwrap_or_default(),
        )
    }

    /// Splits the string by the given delimiter
    ///
    /// # Arguments
    /// * `delimiter` - The string to split on
    #[allow(clippy::option_if_let_else)]
    #[must_use] pub fn split(&self, delimiter: &str) -> Vec<Self> {
        if let Ok(s) = std::str::from_utf8(&self.bytes) {
            s.split(delimiter)
                .map(|s| Self::from(s.to_string()))
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Returns a new string with all occurrences of `from` replaced with `to`
    #[must_use]
    pub fn replace(&self, from: &str, to: &str) -> Self {
        Self::new(
            std::str::from_utf8(&self.bytes)
                .map(|s| s.replace(from, to))
                .unwrap_or_default(),
        )
    }

    /// Returns true if the string contains the given substring
    #[must_use]
    pub fn contains(&self, substr: &str) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.contains(substr))
    }

    /// Returns true if the string starts with the given prefix
    #[must_use]
    pub fn starts_with(&self, prefix: &str) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.starts_with(prefix))
    }

    /// Returns true if the string ends with the given suffix
    #[must_use]
    pub fn ends_with(&self, suffix: &str) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.ends_with(suffix))
    }

    /// Returns true if the string contains only numeric characters
    #[must_use]
    pub fn is_numeric(&self) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.chars().all(char::is_numeric))
    }

    /// Returns true if the string contains only alphabetic characters
    #[must_use]
    pub fn is_alphabetic(&self) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.chars().all(char::is_alphabetic))
    }

    /// Returns true if the string contains only alphanumeric characters
    #[must_use]
    pub fn is_alphanumeric(&self) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.chars().all(char::is_alphanumeric))
    }

    /// Returns true if the string contains only whitespace
    #[must_use]
    pub fn is_whitespace(&self) -> bool {
        std::str::from_utf8(&self.bytes).is_ok_and(|s| s.chars().all(char::is_whitespace))
    }

    /// Performs basic email validation
    ///
    /// Note: This is a basic implementation and should not be used for
    /// production email validation
    #[must_use]
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
    ///
    /// # Errors
    ///
    /// Returns `BStringError::InvalidOperation` if `start` is after `end`, `start` is past the string's end,
    /// or `end` is past the string's end.
    /// Returns `BStringError::InvalidUtf8` if the new string will contain an invalid UTF-8 character
    pub fn substring(&self, start: usize, end: usize) -> Result<Self, BStringError> {
        if start >= self.len() || end > self.len() || start > end {
            return Err(BStringError::InvalidOperation(
                "Invalid substring indices".to_string(),
            ));
        }

        String::from_utf8(self.bytes[start..end].to_vec()).map(Self::new)
            .map_err(|e| BStringError::InvalidUtf8(e.to_string()))
    }

    /// Returns the number of words in the string
    #[must_use]
    pub fn word_count(&self) -> usize {
        std::str::from_utf8(&self.bytes).map_or(0, |s| s.split_whitespace().count())
    }
    #[must_use]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).unwrap_or("")
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
        if let (Ok(s1), Ok(s2)) = (
            std::str::from_utf8(&self.bytes),
            std::str::from_utf8(&other.bytes),
        ) {
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
                let end_pos = first_pos.saturating_add(rhs.len());
                let new_s = format!("{}{}", &s[..first_pos], &s[end_pos..]);
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
        if let (Ok(s1), Ok(s2)) = (
            std::str::from_utf8(&self.bytes),
            std::str::from_utf8(&other.bytes),
        ) {
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
        std::str::from_utf8(&self.bytes.clone()).map_or(self, |s| Self::new(s.repeat(rhs)))
    }
}

/// Count occurences of a substring within a string
impl Div<&str> for BetterString {
    type Output = usize;

    fn div(self, rhs: &str) -> usize {
        if rhs.is_empty() {
            return 0;
        }

        std::str::from_utf8(&self.bytes).map_or(0, |s| {
            let mut count = 0;
            let mut start = 0;

            while let Some(pos) = s[start..].find(rhs) {
                count += 1;
                start += pos + rhs.len();
            }

            count
        })
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
    // Returns a reference to the underlying bytes of the string
    ///
    /// This method provides direct access to the raw bytes that make up the string.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    /// Returns a mutable reference to the underlying bytes of the string
    ///
    /// # Warning
    /// Modifying the bytes directly may lead to invalid UTF-8 sequences
    pub fn as_bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
    /// Consumes the string and returns the underlying byte vector
    ///
    /// This method transfers ownership of the internal bytes to the caller
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
    /// Returns an iterator over the characters of the string
    ///
    /// If the string contains invalid UTF-8, returns an iterator over an empty string
    pub fn chars(&self) -> std::str::Chars<'_> {
        std::str::from_utf8(&self.bytes).map_or_else(|_| "".chars(), |s| s.chars())
    }
    /// Returns an iterator over the characters of the string with their byte indices
    ///
    /// If the string contains invalid UTF-8, returns an iterator over an empty string
    pub fn char_indices(&self) -> std::str::CharIndices<'_> {
        std::str::from_utf8(&self.bytes).map_or_else(|_| "".char_indices(), |s| s.char_indices())
    }
}

// TODO: Add remaining safe methods, make them standard instead of separate
impl BetterString {
    /// Safely splits the string by the given delimiter
    ///
    /// # Arguments
    /// * `delimiter` - The string to split on
    ///
    /// # Returns
    /// * `Ok(Vec<String>)` - A vector of split strings
    /// * `Err(BStringError)` - If the string is empty or the delimiter is invalid
    ///
    /// # Errors
    /// Returns an error if:
    /// * The string is empty
    /// * The delimiter is empty
    /// * The string contains invalid UTF-8
    pub fn safe_split(&self, delimiter: &str) -> Result<Vec<Self>, BStringError> {
        if self.is_empty() {
            return Err(BStringError::EmptyString);
        }
        if delimiter.is_empty() {
            return Err(BStringError::InvalidOperation(
                "Delimiter cannot be empty".to_string(),
            ));
        }

        match std::str::from_utf8(&self.bytes) {
            Ok(s) => Ok(s
                .split(delimiter)
                .map(|s| Self::from(s.to_string()))
                .collect()),
            Err(e) => Err(BStringError::InvalidUtf8(e.to_string())),
        }
    }
}

// Implement Into<String>
impl From<BetterString> for String {
    fn from(val: BetterString) -> Self {
        Self::from_utf8(val.bytes).unwrap_or_default()
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
        Self {
            bytes: s.into_bytes(),
        }
    }
}

impl From<&str> for BetterString {
    fn from(s: &str) -> Self {
        Self {
            bytes: s.as_bytes().to_vec(),
        }
    }
}

impl FromStr for BetterString {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            bytes: s.as_bytes().to_vec(),
        })
    }
}

impl AsRef<[u8]> for BetterString {
    fn as_ref(&self) -> &[u8] {
        &self.bytes
    }
}
impl AsMut<[u8]> for BetterString {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

impl From<Vec<u8>> for BetterString {
    fn from(value: Vec<u8>) -> Self {
        Self { bytes: value }
    }
}

impl Deref for BetterString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        unsafe { std::str::from_utf8_unchecked(&self.bytes) }
    }
}

impl DerefMut for BetterString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { std::str::from_utf8_unchecked_mut(&mut self.bytes) }
    }
}
