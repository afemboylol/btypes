use num_traits::{One, Zero};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, Not, Shl, Shr};

/// A trait that ensures the type supports basic bitwise operations and copying.
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
    + One
    + Zero
    + From<u8>
    + std::cmp::PartialEq
    + Copy
{
}
/// A trait that ensures the type supports basic bitwise operations and cloning.
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
    + From<u8>
    + std::cmp::PartialEq
    + Clone
{
}
/// A trait that ensures the type supports ::zero() and ::one() functions.
pub trait Nums: One + Zero {}

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
