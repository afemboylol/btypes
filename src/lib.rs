#[cfg(feature = "named_bools")]
/// Named boolean collections with fixed capacity
///
/// This module provides BetterBoolNamed types that allow storing and accessing boolean values
/// by name within a fixed-size container. Available in different sizes (8 to 128 bits).
///
/// # Example
/// ```
/// use btypes::named_bools::BN128;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let mut bools = BN128::new();
///     bools.set("flag1", true)?;
///     bools.set("flag2", false)?;
///     Ok(())
/// }
/// ```
pub mod named_bools;

#[cfg(feature = "bools")]
/// Fixed-capacity boolean collections
///
/// This module provides BetterBool types for efficient storage and manipulation of
/// boolean values in fixed-size containers. Available in different sizes (8 to 128 bits).
///
/// # Example
/// ```
/// use btypes::bbool::B128;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let mut bools = B128::new();
///     bools.set_at_pos(0, true)?;
///     bools.set_at_pos(1, false)?;
///     Ok(())
/// }
/// ```
pub mod bbool;

#[cfg(feature = "inf_bools")]
/// Dynamically-sized boolean collections
///
/// This module provides a vector-backed implementation of BetterBool that can grow
/// dynamically as needed, limited only by available memory.
///
/// # Example
/// ```
/// use btypes::inf_bbool::BInf;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let mut bools = BInf::new();
///     bools.set_at_pos(1000, true)?;
///     Ok(())
/// }
/// ```
pub mod inf_bbool;

#[cfg(feature = "inf_named_bools")]
/// Dynamically-sized named boolean collections
///
/// This module provides a vector-backed implementation of BetterBoolNamed that can grow
/// dynamically as needed, allowing named boolean values to be stored and accessed with
/// virtually unlimited capacity.
///
/// # Example
/// ```
/// use btypes::inf_named_bools::BNInf;
/// use anyhow::Result;
///
/// fn main() -> Result<()> {
///     let mut bools = BNInf::new();
///     bools.set("very_large_index", true)?;
///     Ok(())
/// }
/// ```
pub mod inf_named_bools;

/// Common traits for bitwise operations and numeric conversions
///
/// This module defines traits that ensure types support the necessary
/// operations for use in the various boolean collection implementations.
pub mod traits;

#[cfg(feature = "strings")]
/// Enhanced string type with additional functionality
///
/// This module provides BetterString, a more feature-rich alternative to the standard
/// String type, offering additional operations for pattern matching, validation,
/// and encoding conversions.
///
/// # Example
/// ```
/// use btypes::bstring::BetterString;
///
/// let bstr = BetterString::new("Hello, World!");
/// assert!(bstr.matches_pattern(r"^Hello"));
/// ```
pub mod bstring;

/// Error types for the btypes crate
///
/// This module provides specialized error types used throughout the crate's
/// various features and implementations.
pub mod error;

mod readmedoctest;
mod tests;
