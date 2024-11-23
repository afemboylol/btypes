#[cfg(feature = "named_bools")]
/// The feature allowing named variants of BetterBools.
pub mod named_bools;


#[cfg(feature = "bools")]
/// The BetterBool feature.
pub mod bbool;


#[cfg(feature = "inf_bools")]
/// The infinite capacity BetterBool.
pub mod inf_bbool;


#[cfg(feature = "inf_named_bools")]
/// The infinite capacity BetterBoolNamed.
pub mod inf_named_bools;
/// Traits used by different feature's structs.
pub mod traits;


#[cfg(feature = "strings")]
// The BetterString feature.
pub mod bstring;


pub mod error;


mod tests;
