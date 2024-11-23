use num_traits::{One, Zero};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr};

/// A trait that provides a complete set of bitwise operations for types that implement Copy.
///
/// This trait combines several fundamental bitwise operations:
/// - Bitwise AND (&), OR (|), XOR (^)
/// - Left shift (<<) and right shift (>>)
/// - Bitwise AND assignment (&=) and OR assignment (|=)
/// - Bitwise NOT (!)
///
/// Additionally, it requires:
/// - `Copy` semantics for efficient value duplication
/// - Conversion from `u8`
/// - Constants for 0 and 1 via `Zero` and `One` traits from `Nums` trait
/// - Partial equality comparison
///
/// Commonly implemented for fixed-size unsigned and signed integer types.
pub trait BitwiseOpsCopy:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<u8, Output = Self>
    + Shr<u8, Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
    + Sized
    + Nums
    + From<u8>
    + std::cmp::PartialEq
    + Copy
{
}

/// A trait that provides a complete set of bitwise operations for types that implement Clone.
///
/// Similar to `BitwiseOpsCopy` but for types that require explicit cloning.
/// Useful for types where copying is more expensive or semantically inappropriate.
///
/// Includes all bitwise operations:
/// - Bitwise AND (&), OR (|), XOR (^)
/// - Left shift (<<) and right shift (>>)
/// - Bitwise AND assignment (&=) and OR assignment (|=)
/// - Bitwise NOT (!)
///
/// The only difference from `BitwiseOpsCopy` is the `Clone` bound instead of `Copy`.
pub trait BitwiseOpsClone:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<u8, Output = Self>
    + Shr<u8, Output = Self>
    + BitAndAssign
    + BitOrAssign
    + Not<Output = Self>
    + Sized
    + Nums
    + From<u8>
    + std::cmp::PartialEq
    + Clone
{
}

/// A trait for types that can represent both zero and one.
///
/// This trait combines the `Zero` and `One` traits from `num_traits`,
/// providing a convenient way to require both capabilities in a single bound.
/// Useful for numeric types that need to represent binary states or perform
/// basic arithmetic operations.
pub trait Nums: One + Zero {}

/// A simple trait that, if implemented on any type, allows for it to be used entirely with BetterBool and BetterBoolNamed.
pub trait BoolSupport: BitwiseOpsClone + BitwiseOpsCopy {}

impl BitwiseOpsCopy for u128 {}
impl BitwiseOpsCopy for u64 {}
impl BitwiseOpsCopy for u32 {}
impl BitwiseOpsCopy for u16 {}
impl BitwiseOpsCopy for u8 {}

impl BitwiseOpsCopy for i128 {}
impl BitwiseOpsCopy for i64 {}
impl BitwiseOpsCopy for i32 {}
impl BitwiseOpsCopy for i16 {}

impl BitwiseOpsClone for u128 {}
impl BitwiseOpsClone for u64 {}
impl BitwiseOpsClone for u32 {}
impl BitwiseOpsClone for u16 {}
impl BitwiseOpsClone for u8 {}

impl BitwiseOpsClone for i128 {}
impl BitwiseOpsClone for i64 {}
impl BitwiseOpsClone for i32 {}
impl BitwiseOpsClone for i16 {}

impl Nums for u128 {}
impl Nums for u64 {}
impl Nums for u32 {}
impl Nums for u16 {}
impl Nums for u8 {}
impl Nums for i128 {}
impl Nums for i64 {}
impl Nums for i32 {}
impl Nums for i16 {}
impl Nums for i8 {}
