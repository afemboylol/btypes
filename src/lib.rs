//#[cfg(feature = "named_bools")]
/// The feature allowing named variants of BetterBools.
pub mod named_bools;
//#[cfg(feature = "bools")]
/// The BetterBool feature.
pub mod bbool;
/// Traits used by different feature's structs.
pub mod traits;
#[cfg(feature = "strings")]
pub mod bstring;

pub mod error;

mod tests;
