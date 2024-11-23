use anyhow::{Error, Result};
use std::marker::PhantomData;
use crate::traits::{BitwiseOpsClone, BitwiseOpsCopy, Nums};

pub type B128 = BetterBool<u128>;
pub type B64 = BetterBool<u64>;
pub type B32 = BetterBool<u32>;
pub type B16 = BetterBool<u16>;
pub type B8 = BetterBool<u8>;
pub type BBool<T> = BetterBool<T>;

pub struct BetterBool<T: Nums> {
    pub(crate) store: T,
    pub(crate) reader_head_pos: u8,
    pub(crate) _marker: PhantomData<T>,
}

impl<T: Nums> Default for BetterBool<T> {
    fn default() -> Self {
        Self {
            store: T::zero(),
            reader_head_pos: 0,
            _marker: PhantomData,
        }
    }
}

impl<T: Nums> BetterBool<T> {
    pub const CAP: u8 = (size_of::<T>() * 8) as u8;
}

impl<T: Nums + BitwiseOpsCopy> BetterBool<T> {
    /// Creates a new empty BetterBool instance initialized with zeros.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B128;
    /// let bools = B128::new();
    /// ```
    pub fn new() -> Self {
        Self {
            store: T::zero(),
            reader_head_pos: 0,
            _marker: PhantomData,
        }
    }

    /// Creates a new BetterBool instance with a specified initial value.
    ///
    /// # Arguments
    /// * `initial_value` - The initial numeric value to store the boolean states
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B128;
    /// let bools = B128::from_num(42);
    /// ```
    pub fn from_num(initial_value: T) -> Self {
        Self {
            store: initial_value,
            reader_head_pos: 0,
            _marker: PhantomData,
        }
    }

    /// Returns a Vec of all bools in the container.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::from_num(5);
    /// let all_bools = bools.all()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if accessing any position fails
    pub fn all(&mut self) -> Result<Vec<bool>> {
        let mut out = vec![];
        for i in 0..Self::CAP {
            out.push(self.get_at_pos(i.try_into()?)?);
        }
        Ok(out)
    }

    /// Returns a new BetterBool<T> that has been sorted.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::from_num(5);
    /// let sorted = bools.sorted()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if sorting operation fails
    pub fn sorted(&mut self) -> Result<BetterBool<T>> {
        let mut bools = self.all()?;
        bools.sort();

        let mut sorted = BetterBool::new();
        for (i, &value) in bools.iter().enumerate() {
            sorted.set_at_pos(i as u8, value)?;
        }
        Ok(sorted)
    }

    /// Gets the bool at the current head position. (doesn't clone self.store)
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = B8::from_num(5);
    /// let value = bools.get()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if head position is invalid
    pub fn get(&self) -> Result<bool> {
            if self.reader_head_pos < Self::CAP {
                let mask = T::one() << self.reader_head_pos.into();
                return Ok((self.store & mask) != T::zero());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Gets the bool at the given position. (doesn't clone self.store)
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = B8::from_num(5);
    /// let value = bools.get_at_pos(2)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if position is invalid
    pub fn get_at_pos(&self, pos: u8) -> Result<bool> {
        if self.reader_head_pos < Self::CAP {
                let mask = T::one() << pos;
                return Ok((self.store & mask) != T::zero());
            }
    
        Err(Error::msg("Invalid position"))
    }

    /// Gets the bool at the current head position without validity checks. (doesn't clone self.store)
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the head position is valid.
    pub unsafe fn get_unchecked(&self) -> bool {
        let mask = T::one() << self.reader_head_pos.into();
        (self.store & mask) != T::zero()
    }

    /// Gets the bool at the given position without validity checks. (doesn't clone self.store)
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the position is valid.
    pub unsafe fn get_unchecked_at_pos(&self, pos: u8) -> bool {
        let mask = T::one() << pos;
        (self.store & mask) != T::zero()
    }

    /// Get an immutable reference to the bools contained in a raw binary format.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// let bools = B8::from_num(5);
    /// let raw = bools.get_raw();
    /// ```
    pub fn get_raw(&self) -> &T {
        &self.store
    }

    /// Get a mutable reference to the bools contained in a raw binary format.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// let mut bools = B8::from_num(5);
    /// let raw_mut = bools.get_raw_mut();
    /// ```
    pub fn get_raw_mut(&mut self) -> &mut T {
        &mut self.store
    }

    /// Sets the bool at the current head position.
    ///
    /// # Arguments
    /// * `new` - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::new();
    /// bools.set(true);
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if head position is invalid
    pub fn set(&mut self, new: bool) -> Result<()> {
            if self.reader_head_pos < Self::CAP {
                let mask = T::one() << self.reader_head_pos.into();
                if new {
                    self.store |= mask; // Set the bit using OR
                } else {
                    self.store &= !mask; // Clear the bit using AND with NOT
                }
                return Ok(());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Sets the bool at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position to set
    /// * `new` - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::new();
    /// bools.set_at_pos(2, true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if position is invalid
    pub fn set_at_pos(&mut self, pos: u8, new: bool) -> Result<()> {
            if pos < Self::CAP {
                let mask = T::one() << pos;
                if new {
                    self.store |= mask; // Set the bit using OR
                } else {
                    self.store &= !mask; // Clear the bit using AND with NOT
                }
                return Ok(());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Sets the bool at the current head position without validity checks.
    ///
    /// # Arguments
    /// * `new` - The boolean value to set
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the head position is valid.
    pub unsafe fn set_unchecked(&mut self, new: bool) {
        let mask = T::one() << self.reader_head_pos.into();
        if new {
            self.store |= mask; // Set the bit using OR
        } else {
            self.store &= !mask; // Clear the bit using AND with NOT
        }
    }

    /// Sets the bool at the given position without validity checks.
    ///
    /// # Arguments
    /// * `pos` - The position to set
    /// * `new` - The boolean value to set
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the position is valid.
    pub unsafe fn set_unchecked_at_pos(&mut self, pos: u8, new: bool) {
        let mask = T::one() << pos;
        if new {
            self.store |= mask; // Set the bit using OR
        } else {
            self.store &= !mask; // Clear the bit using AND with NOT
        }
    }

    /// Gets the value at the current head position and increments the head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::from_num(5);
    /// let value = bools.next()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Getting the current value fails
    /// * Incrementing the head position fails
    pub fn next(&mut self) -> Result<bool> {
        let val = self.get()?;
        self.inc()?;
        Ok(val)
    }

    /// Gets the value at the current head position, wipes it, and increments the head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::from_num(5);
    /// let value = bools.next_res()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Getting the current value fails
    /// * Setting the value fails
    /// * Incrementing the head position fails
    pub fn next_res(&mut self) -> Result<bool> {
        let val = self.get()?;
        self.set(false)?;
        self.inc()?;
        Ok(val)
    }

    /// Increments the head position by 1.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::new();
    /// bools.inc()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the new head position would be invalid
    pub fn inc(&mut self) -> Result<()> {
            if self.reader_head_pos + 1 < Self::CAP {
                self.reader_head_pos += 1;
                return Ok(());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Increments the head position by 1 without validity checks.
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the new head position will be valid.
    pub unsafe fn inc_unchecked(&mut self) {
        self.reader_head_pos += 1;
    }

    /// Sets the head position without validity checks.
    ///
    /// # Arguments
    /// * `new` - The new head position
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the new head position is valid.
    pub unsafe fn shp_unchecked(&mut self, new: u8) {
        self.reader_head_pos = new;
    }

    /// Sets the head position to be a new value
    ///
    /// # Arguments
    /// * `new` - The new head position
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = B8::new();
    /// bools.shp(2)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the new head position is invalid
    pub fn shp(&mut self, new: u8) -> Result<()> {
            if new < Self::CAP {
                self.reader_head_pos = new;
                return Ok(());
        }
        Err(Error::msg("Invalid head position"))
    }

    /// Gets an immutable reference to the current head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// let bools = B8::new();
    /// let head_pos = bools.ghp();
    /// ```
    pub fn ghp(&self) -> &u8 {
        return &self.reader_head_pos;
    }

    /// Gets a mutable reference to the current head position (use disrecommended).
    ///
    /// # Warning
    /// Direct manipulation of the head position is not recommended as it bypasses validity checks.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// let mut bools = B8::new();
    /// let head_pos_mut = bools.ghp_mut();
    /// ```
    pub fn ghp_mut(&mut self) -> &mut u8 {
        return &mut self.reader_head_pos;
    }
    pub fn clear(&mut self)
    {
        self.store = T::zero();
    }
}

impl<T: BitwiseOpsClone + Nums> BetterBool<T> {
    /// Gets the bool at the current head position (clones self.store).
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = B8::from_num(5);
    /// let value = bools.get_cl()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if head position is invalid
    pub fn get_cl(&self) -> Result<bool> {
            if self.reader_head_pos < Self::CAP {
                let mask = T::one() << self.reader_head_pos.into();
                return Ok((self.store.clone() & mask) != T::zero());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Gets the bool at the current head position without validity checks. (clones self.store)
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the head position is valid.
    pub unsafe fn get_unchecked_cl(&self) -> bool {
        let mask = T::one() << self.reader_head_pos.into();
        (self.store.clone() & mask) != T::zero()
    }

    /// Gets the bool at the given position (clones self.store).
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = B8::from_num(5);
    /// let value = bools.get_cl_at_pos(2)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if position is invalid
    pub fn get_cl_at_pos(&self, pos: u8) -> Result<bool> {
            if pos < Self::CAP {
                let mask = T::one() << pos;
                return Ok((self.store.clone() & mask) != T::zero());
            }
        Err(Error::msg("Invalid head position"))
    }

    /// Gets the bool at the given position without validity checks. (clones self.store)
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the position is valid.
    pub unsafe fn get_unchecked_cl_at_pos(&self, pos: u8) -> bool {
        let mask = T::one() << pos;
        (self.store.clone() & mask) != T::zero()
    }

    /// Get a clone of the bools contained in a raw numeric format.
    ///
    /// # Examples
    /// ```
    /// use btypes::bbool::B8;
    /// let bools = B8::from_num(5);
    /// let raw_clone = bools.get_raw_cl();
    /// ```
    pub fn get_raw_cl(&self) -> T {
        self.store.clone()
    }
}
