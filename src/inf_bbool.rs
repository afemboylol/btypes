use crate::error::BBoolError;
use anyhow::Result;
use std::fmt::Display;
use std::marker::PhantomData;

/// Type alias for the infinite-capacity `BetterBool` implementation
pub type BInf = BetterBoolInf;

/// A dynamically-sized boolean collection backed by a vector
///
/// This struct provides storage and operations for boolean values with
/// virtually unlimited capacity, growing as needed.
#[derive(Clone, Debug)]
pub struct BetterBoolInf {
    /// The vector storing the boolean bits as bytes
    pub(crate) store: Vec<u8>,
    /// Current position of the reader head
    pub(crate) reader_head_pos: usize,
    /// Phantom data for the vector type
    pub(crate) _marker: PhantomData<Vec<u8>>,
}

impl Default for BetterBoolInf {
    fn default() -> Self {
        Self {
            store: Vec::new(),
            reader_head_pos: 0,
            _marker: PhantomData,
        }
    }
}

impl BetterBoolInf {
    /// The limit of the "Infinite" `BetterBool`.Uunfortunately finite, due to limitations of the head position (limited to u128::MAX theoretically) and the max Vec size (usize::MAX) without unnecessary complexity.
    pub const CAP: usize = usize::MAX;
}

impl BetterBoolInf {
    /// Creates a new empty `BetterBoolInf` instance initialized with an empty vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new empty `BetterBoolInf` instance initialized with a vector with the specified capacity.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::with_cap(8); // 8 bools
    /// ```
    #[must_use]
    pub fn with_cap(cap: usize) -> Self {
        Self {
            store: Vec::with_capacity(cap / 8),
            reader_head_pos: 0,
            _marker: PhantomData,
        }
    }

    /// Returns a Vec of boolean values within the specified range [start, end).
    ///
    /// # Arguments
    /// * `start` - The starting position (inclusive)
    /// * `end` - The ending position (exclusive)
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    ///     let mut bools = BInf::new();
    ///     bools.set_at_pos(0, true)?;
    ///     bools.set_at_pos(1, false)?;
    ///     let range_bools = bools.range(0, 2)?;
    ///     assert_eq!(range_bools, vec![true, false]);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * start position is invalid
    /// * end position is invalid
    /// * end is less than start
    /// * accessing any position in range fails
    pub fn range(&self, start: usize, end: usize) -> Result<Vec<bool>, BBoolError> {
        if start >= Self::CAP {
            return Err(BBoolError::InvalidPosInf(start));
        }
        if end > Self::CAP {
            return Err(BBoolError::InvalidPosInf(end));
        }
        if end < start {
            return Err(BBoolError::InvalidRange(start, end));
        }

        let mut result = Vec::with_capacity(end - start);
        for pos in start..end {
            result.push(self.get_at_pos(pos)?);
        }
        Ok(result)
    }

    /// Returns the current capacity of the internal vector, in bits.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::with_cap(8); // 8 bools
    /// println!("{}", bools.cap());
    /// assert!(bools.cap() == 8);
    /// ```
    #[must_use]
    pub fn cap(&self) -> usize {
        self.store.capacity() * 8
    }

    /// Creates a new `BetterBoolInf` instance with a specified initial vector of bytes.
    ///
    /// # Arguments
    /// * `initial_value` - The initial vector of bytes to store the boolean states
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::from_vec(vec![42]);
    /// ```
    #[must_use]
    pub const fn from_vec(initial_value: Vec<u8>) -> Self {
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
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::from_vec(vec![5]);
    /// let all_bools = bools.all()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if accessing any position fails
    pub fn all(&self) -> Result<Vec<bool>, BBoolError> {
        let mut out = vec![];
        // Multiply by 8 since each byte contains 8 bits
        for i in 0..(self.store.len() * 8) {
            out.push(self.get_at_pos(i)?);
        }
        Ok(out)
    }

    /// Returns a new `BetterBoolInf` that has been sorted.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::from_vec(vec![5]);
    /// let sorted = bools.sorted()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if sorting operation fails
    pub fn sorted(&self) -> Result<Self, BBoolError> {
        let mut bools = self.all()?;
        bools.sort_unstable();

        let mut sorted = Self::new();
        for (i, &value) in bools.iter().enumerate() {
            sorted.set_at_pos(i, value)?;
        }
        Ok(sorted)
    }

    /// Gets the bool at the current head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = BInf::from_vec(vec![5]);
    /// let value = bools.get()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if head position is invalid
    pub fn get(&self) -> Result<bool, BBoolError> {
        if self.reader_head_pos < Self::CAP {
            let byte_index = self.reader_head_pos / 8;
            let bit_offset = self.reader_head_pos % 8;

            if byte_index >= self.store.len() {
                return Ok(false); // Return false for unallocated positions
            }

            let mask = 1u8 << bit_offset;
            return Ok((self.store[byte_index] & mask) != 0);
        }
        Err(BBoolError::InvalidHeadPosInf(self.reader_head_pos))
    }

    /// Gets the bool at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let bools = BInf::from_vec(vec![5]);
    /// let value = bools.get_at_pos(2)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if position is invalid
    pub fn get_at_pos(&self, pos: usize) -> Result<bool, BBoolError> {
        if pos < Self::CAP {
            let byte_index = pos / 8;
            let bit_offset = pos % 8;

            if byte_index >= self.store.len() {
                return Ok(false); // Return false for unallocated positions
            }

            let mask = 1u8 << bit_offset;
            return Ok((self.store[byte_index] & mask) != 0);
        }
        Err(BBoolError::InvalidPosInf(pos))
    }

    /// Gets the bool at the current head position without validity checks.
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the head position is valid.
    #[must_use]
    pub unsafe fn get_unchecked(&self) -> bool {
        let byte_index = self.reader_head_pos / 8;
        let bit_offset = self.reader_head_pos % 8;

        if byte_index >= self.store.len() {
            return false;
        }

        let mask = 1u8 << bit_offset;
        (self.store[byte_index] & mask) != 0
    }

    /// Gets the bool at the given position without validity checks.
    ///
    /// # Arguments
    /// * `pos` - The position to read from
    ///
    /// # Safety
    /// This function performs no bounds checking. the position is valid.
    #[must_use]
    pub unsafe fn get_unchecked_at_pos(&self, pos: usize) -> bool {
        let byte_index = pos / 8;
        let bit_offset = pos % 8;

        if byte_index >= self.store.len() {
            return false;
        }

        let mask = 1u8 << bit_offset;
        (self.store[byte_index] & mask) != 0
    }

    /// Sets the bool at the current head position without validity checks.
    ///
    /// # Arguments
    /// * `new` - The boolean value to set
    ///
    /// # Safety
    /// This function performs no bounds checking. The caller must ensure the head position is valid.
    pub unsafe fn set_unchecked(&mut self, new: bool) {
        let byte_index = self.reader_head_pos / 8;
        let bit_offset = self.reader_head_pos % 8;

        while byte_index >= self.store.len() {
            self.store.push(0);
        }

        let mask = 1u8 << bit_offset;
        if new {
            self.store[byte_index] |= mask;
        } else {
            self.store[byte_index] &= !mask;
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
    pub unsafe fn set_unchecked_at_pos(&mut self, pos: usize, new: bool) {
        let byte_index = pos / 8;
        let bit_offset = pos % 8;

        while byte_index >= self.store.len() {
            self.store.push(0);
        }

        let mask = 1u8 << bit_offset;
        if new {
            self.store[byte_index] |= mask;
        } else {
            self.store[byte_index] &= !mask;
        }
    }

    /// Get an immutable reference to the bools contained in a raw byte vector format.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::from_vec(vec![5]);
    /// let raw = bools.get_raw();
    /// ```
    #[must_use]
    pub const fn get_raw(&self) -> &Vec<u8> {
        &self.store
    }

    /// Get a mutable reference to the bools contained in a raw byte vector format.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let mut bools = BInf::from_vec(vec![5]);
    /// let raw_mut = bools.get_raw_mut();
    /// ```
    pub fn get_raw_mut(&mut self) -> &mut Vec<u8> {
        &mut self.store
    }

    /// Sets the bool at the current head position.
    ///
    /// # Arguments
    /// * `new` - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::new();
    /// bools.set(true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if head position is invalid
    pub fn set(&mut self, new: bool) -> Result<(), BBoolError> {
        if self.reader_head_pos < Self::CAP {
            let byte_index = self.reader_head_pos / 8;
            let bit_offset = self.reader_head_pos % 8;

            // Extend the vector if necessary
            while byte_index >= self.store.len() {
                self.store.push(0);
            }

            let mask = 1u8 << bit_offset;
            if new {
                self.store[byte_index] |= mask;
            } else {
                self.store[byte_index] &= !mask;
            }
            return Ok(());
        }
        Err(BBoolError::InvalidHeadPosInf(self.reader_head_pos))
    }

    /// Sets the bool at the given position.
    ///
    /// # Arguments
    /// * `pos` - The position to set
    /// * `new` - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::new();
    /// bools.set_at_pos(2, true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if position is invalid
    pub fn set_at_pos(&mut self, pos: usize, new: bool) -> Result<(), BBoolError> {
        if pos < Self::CAP {
            let byte_index = pos / 8;
            let bit_offset = pos % 8;

            // Extend the vector if necessary
            while byte_index >= self.store.len() {
                self.store.push(0);
            }

            let mask = 1u8 << bit_offset;
            if new {
                self.store[byte_index] |= mask;
            } else {
                self.store[byte_index] &= !mask;
            }
            return Ok(());
        }
        Err(BBoolError::InvalidPosInf(pos))
    }

    /// Gets the value at the current head position and increments the head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::from_vec(vec![5]);
    /// let value = bools.next_b()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Getting the current value fails
    /// * Incrementing the head position fails
    pub fn next_b(&mut self) -> Result<bool, BBoolError> {
        let val = self.get()?;
        self.inc()?;
        Ok(val)
    }

    /// Gets the value at the current head position, wipes it, and increments the head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::from_vec(vec![5]);
    /// let value = bools.next_b_res()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Getting the current value fails
    /// * Setting the value fails
    /// * Incrementing the head position fails
    pub fn next_b_res(&mut self) -> Result<bool, BBoolError> {
        let val = self.get()?;
        self.set(false)?;
        self.inc()?;
        Ok(val)
    }

    /// Increments the head position by 1.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::new();
    /// bools.inc()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the new head position would be invalid
    pub fn inc(&mut self) -> Result<(), BBoolError> {
        if self.reader_head_pos + 1 < Self::CAP {
            self.reader_head_pos += 1;
            return Ok(());
        }
        Err(BBoolError::InvalidHeadPosInf(self.reader_head_pos))
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
    pub unsafe fn shp_unchecked(&mut self, new: usize) {
        self.reader_head_pos = new;
    }

    /// Sets the head position to a new value.
    ///
    /// # Arguments
    /// * new - The new head position
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BInf::new();
    /// bools.shp(42)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the new head position would be invalid
    pub fn shp(&mut self, new: usize) -> Result<(), BBoolError> {
        if new < Self::CAP {
            self.reader_head_pos = new;
            return Ok(());
        }
        Err(BBoolError::InvalidHeadPosInf(new))
    }

    /// Gets an immutable reference to the current head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let bools = BInf::new();
    /// let head_pos = bools.ghp();
    /// ```
    ///
    #[must_use]
    pub const fn ghp(&self) -> &usize {
        &self.reader_head_pos
    }

    /// Gets a mutable reference to the current head position.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let mut bools = BInf::new();
    /// let head_pos_mut = bools.ghp_mut();
    /// ```
    ///
    pub fn ghp_mut(&mut self) -> &mut usize {
        &mut self.reader_head_pos
    }

    /// Clears all stored boolean values.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_bbool::BInf;
    /// let mut bools = BInf::from_vec(vec![5]);
    /// bools.clear();
    /// ```
    ///
    pub fn clear(&mut self) {
        self.store.clear();
    }
}

impl Display for BetterBoolInf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.all())
    }
}

impl IntoIterator for BetterBoolInf {
    type Item = bool;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.all()
            .expect("Failed to get all bool values contained")
            .into_iter()
    }
}
