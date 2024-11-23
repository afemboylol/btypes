use crate::bbool::BetterBool;
use crate::error::BBoolError;
use crate::traits::{BitwiseOpsClone, BitwiseOpsCopy, Nums};
use anyhow::Error;
use anyhow::Result;
use std::{collections::HashMap, marker::PhantomData};

/// Type alias for a 128-bit named `BetterBool`
pub type BN128 = BetterBoolNamed<u128>;
/// Type alias for a 64-bit named `BetterBool`
pub type BN64 = BetterBoolNamed<u64>;
/// Type alias for a 32-bit named `BetterBool`
pub type BN32 = BetterBoolNamed<u32>;
/// Type alias for a 16-bit named `BetterBool`
pub type BN16 = BetterBoolNamed<u16>;
/// Type alias for an 8-bit named `BetterBool`
pub type BN8 = BetterBoolNamed<u8>;
/// Generic type alias for named `BetterBool` with any numeric type T
pub type BNBool<T> = BetterBoolNamed<T>;

/// A fixed-size collection of named boolean values
///
/// This struct combines the fixed-size storage of `BetterBool` with the ability
/// to access boolean values by name rather than position.
#[derive(Clone, Debug)]
pub struct BetterBoolNamed<T: Nums> {
    /// The underlying boolean storage
    pub bools: BetterBool<T>,
    /// Mapping of names to boolean positions
    names: HashMap<String, u8>,
    /// Next available position for new boolean values
    next_assign: u8,
}

impl<T: Nums> Default for BetterBoolNamed<T>
{
    fn default() -> Self {
        Self
        {
            bools: BetterBool::default(),
            names: HashMap::new(),
            next_assign: 0,
        }
    }
}

impl<T: BitwiseOpsCopy> BetterBoolNamed<T> {
    /// Creates a new `BetterBoolNamed` instance with a specified initial value.
    ///
    /// # Arguments
    /// * `initial_value` - The initial numeric value to store the boolean states
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::from_num(0b01010110);
    /// ```
    pub fn from_num(initial_value: T) -> Self {
        let bools = BetterBool::<T> {
            store: initial_value,
            reader_head_pos: 0,
            _marker: PhantomData,
        };
        Self {
            bools,
            names: HashMap::new(),
            next_assign: 0,
        }
    }
    /// Creates a new empty `BetterBoolNamed` instance initialized with zeros.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::new();
    /// ```
    #[must_use] pub fn new() -> Self {
        Self::default()
    }
    /// Set/add many named bools, with the names being dictated by the pattern and the values by the value pattern.
    ///
    /// # Arguments
    /// * `count` - Number of bools to set/add
    /// * `pattern` - Name pattern containing {n} which will be replaced with sequential numbers (0 to count-1)
    /// * `value_pattern` - Comma-separated list of boolean values with optional {r} suffix to repeat the pattern (if list length does not contain {r}, or exceed)
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// fn main() -> anyhow::Result<()> {
    /// let mut bools = BN128::new();
    /// // Creates bool_0=true, bool_1=false, bool_2=true, bool_3=false, etc.
    /// bools.mass_set(4, "bool_{n}", "true,false{r}")?;
    ///
    /// // Creates test0=true, test1=true, test2=true
    /// bools.mass_set(3, "test{n}", "true{r}")?;
    ///
    /// // Creates val_0=true, val_1=false, val_2=true, val_3=true
    /// bools.mass_set(4, "val_{n}", "true,false,true,true")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The pattern doesn't contain {n}
    /// * The value pattern is empty
    /// * The value pattern doesn't contain {r} and the count of bools in it doesn't match or exceed the count.
    /// * The value pattern contains invalid boolean values
    /// * Adding the bools would exceed capacity
    pub fn mass_set(
        &mut self,
        count: u8,
        pattern: &str,
        value_pattern: &str,
    ) -> Result<(), BBoolError> {
        // Validate pattern contains {n}
        if !pattern.contains("{n}") {
            return Err(BBoolError::InvalidPattern(
                "Pattern must contain {n}".to_string(),
            ));
        }

        // Parse value pattern
        let value_parts: Vec<&str> = value_pattern.trim().split(',').collect();
        if value_parts.is_empty() {
            return Err(BBoolError::InvalidPattern(
                "Value pattern cannot be empty".to_string(),
            ));
        }
        if !value_pattern.contains("{r}") && value_parts.len() < count.into() {
            println!("{}, {}", !value_parts.contains(&"{r}"), value_parts.len());
            return Err(BBoolError::InvalidPattern(
                "Value pattern must be able to fill all set bools".to_string(),
            ));
        }

        let repeating = value_pattern.ends_with("{r}");
        let values: Vec<bool> = value_parts
            .iter()
            .map(|&s| s.trim().trim_end_matches("{r}"))
            .map(|s| match s.to_lowercase().as_str() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(Error::msg("Invalid boolean value in pattern")),
            })
            .collect::<Result<Vec<bool>>>()?;

        // Set/add bools
        for i in 0..count {
            let name = pattern.replace("{n}", &i.to_string());
            let value_index = if repeating {
                (i as usize) % values.len()
            } else {
                if i as usize >= values.len() {
                    let last = values.last().ok_or_else(|| BBoolError::Other("Failed to get last element of values.".to_string()))?;
                    self.set(&name, *last)?;
                    continue;
                }
                i as usize
            };
            self.set(&name, values[value_index])?;
        }

        Ok(())
    }
    /// Gets multiple boolean values associated with the given names.
    ///
    /// # Arguments
    /// * names - A slice of string slices containing the names to retrieve
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test1", true)?;
    /// bools.add("test2", false)?;
    /// let values = bools.mass_get(&vec!["test1", "test2"])?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Any of the names don't exist in the collection
    /// * Retrieving any value fails
    pub fn mass_get(&self, names: &[&str]) -> Result<Vec<bool>> {
        let mut out = Vec::with_capacity(names.len());
        for name in names {
            out.push(self.get(name)?);
        }
        Ok(out)
    }
    /// Toggles multiple boolean values associated with the given names.
    ///
    /// # Arguments
    /// * names - A slice of strings containing the names of the values to toggle
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test1", true)?;
    /// bools.add("test2", false)?;
    /// bools.mass_toggle(&vec!["test1", "test2"])?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Any of the names don't exist in the collection
    /// * Toggling any value fails
    pub fn mass_toggle(&mut self, names: &[&str]) -> Result<()> {
        for name in names {
            self.toggle(name)?;
        }
        Ok(())
    }
    /// Sorts the current instance in place by name.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("c", true)?;
    /// bools.add("a", false)?;
    /// bools.add("b", true)?;
    /// bools.sort()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the sorting operation fails
    pub fn sort(&mut self) -> Result<()> {
        let b = self.sorted()?;
        self.names = b.names;
        self.bools = b.bools;
        Ok(())
    }
    /// Returns a new `BetterBoolNamed` instance with contents sorted by name.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("c", true)?;
    /// bools.add("a", false)?;
    /// let sorted_bools = bools.sorted()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the sorting operation fails
    pub fn sorted(&self) -> Result<Self> {
        // Get all name-value pairs and sort them by name
        let mut pairs: Vec<_> = self.all()?.into_iter().collect();
        pairs.sort_by(|(a, _), (b, _)| a.cmp(b));

        // Create new instance
        let mut sorted = Self::new();

        // Add sorted pairs in order
        for (name, value) in pairs {
            sorted.add(&name, value)?;
        }

        Ok(sorted)
    }
    /// Returns all boolean values in the collection as a vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test1", true)?;
    /// bools.add("test2", false)?;
    /// let all_values = bools.all_bools()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if retrieving any boolean value fails
    pub fn all_bools(&self) -> Result<Vec<bool>> {
        self.bools.all()
    }
    /// Returns a clone of the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::new();
    /// let names = bools.all_names_cl();
    /// ```
    pub fn all_names_cl(&self) -> HashMap<String, u8> {
        self.names.clone()
    }
    /// Returns a reference to the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::new();
    /// let names = bools.all_names();
    /// ```
    pub const fn all_names(&self) -> &HashMap<String, u8> {
        &self.names
    }
    /// Returns a mutable reference to the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let mut bools = BN128::new();
    /// let names_mut = bools.all_names_mut();
    /// ```
    pub fn all_names_mut(&mut self) -> &mut HashMap<String, u8> {
        &mut self.names
    }
    /// Returns a `HashMap` containing all name-value pairs in the collection.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// let all_pairs = bools.all()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if retrieving any boolean value fails
    pub fn all(&self) -> Result<HashMap<String, bool>> {
        let mut result = HashMap::new();
        for (name, &position) in &self.names {
            result.insert(name.clone(), self.bools.get_at_pos(position)?);
        }
        Ok(result)
    }

    /// Sets or adds a boolean value with the given name.
    ///
    /// # Arguments
    /// * `name` - The name to associate with the boolean value
    /// * `value` - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.set("test", true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Setting the value fails
    /// * Adding a new value fails
    pub fn set(&mut self, name: &str, value: bool) -> Result<()> {
        match self.names.get(name) {
            Some(&position) => self.bools.set_at_pos(position, value)?,
            None => self.add(name, value)?,
        }
        Ok(())
    }
    /// Toggles the boolean value associated with the given name.
    ///
    /// # Arguments
    /// * name - The name of the boolean value to toggle
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// bools.toggle("test")?; // Value is now false
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The name doesn't exist in the collection
    /// * Setting the toggled value fails
    pub fn toggle(&mut self, name: &str) -> Result<()> {
        let current = self.get(name)?;
        self.set(name, !current)?;
        Ok(())
    }

    /// Checks if a boolean value with the given name exists in the collection.
    ///
    /// # Arguments
    /// * `name` - The name to check for existence
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let mut bools = BN128::new();
    /// let exists = bools.exists("test");
    /// ```
    pub fn exists(&self, name: &str) -> bool {
        self.names.contains_key(name)
    }
    /// Gets an immutable reference to the raw numeric storage.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::new();
    /// let raw = bools.get_raw();
    /// ```
    pub const fn get_raw(&self) -> &T {
        self.bools.get_raw()
    }
    /// Gets a mutable reference to the raw numeric storage.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let mut bools = BN128::new();
    /// let raw_mut = bools.get_raw_mut();
    /// ```
    pub fn get_raw_mut(&mut self) -> &mut T {
        self.bools.get_raw_mut()
    }
    /// Adds a new boolean value with the given name to the collection.
    ///
    /// # Arguments
    /// * `name` - The name to associate with the boolean value
    /// * `value` - The boolean value to add
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The collection has128 items)
    /// * Setting the value fails
    pub fn add(&mut self, name: &str, value: bool) -> Result<(), BBoolError> {
        if self.names.len() >= 128 {
            return Err(BBoolError::CollectionCapacityReached);
        }
        self.names.insert(name.to_string(), self.next_assign);
        self.bools.set_at_pos(self.next_assign, value)?;
        self.next_assign += 1;
        Ok(())
    }

    /// Gets the boolean value associated with the given name.
    ///
    /// # Arguments
    /// * `name` - The name of the boolean value to retrieve
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// let value = bools.get("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The name doesn't exist in the collection
    /// * Retrieving the value fails
    pub fn get(&self, name: &str) -> Result<bool, BBoolError> {
        match self.names.get(name) {
            Some(&position) => Ok(self.bools.get_at_pos(position)?),
            None => Err(BBoolError::NotFound(name.to_string())),
        }
    }
    /// Deletes a boolean value from the collection.
    ///
    /// # Arguments
    /// * name - The name of the boolean value to delete
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// bools.delete("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * Setting the value to false before deletion fails
    pub fn delete(&mut self, name: &str) -> Result<()> {
        if self.names.contains_key(name) {
            self.set(name, false)?;
            self.names.remove(name);
        }
        Ok(())
    }
    /// Clears all stored boolean values and associated names.
    pub fn clear(&mut self) {
        self.names.clear();
        self.bools.clear();
    }
}
impl<T: BitwiseOpsClone> BetterBoolNamed<T> {
    /// Gets the boolean value associated with the given name, using cloning.
    ///
    /// # Arguments
    /// * `name` - The name of the boolean value to retrieve
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BN128::new();
    /// bools.add("test", true)?;
    /// let value = bools.get_cl("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The name doesn't exist in the collection
    /// * Retrieving the value fails
    pub fn get_cl(&self, name: &str) -> Result<bool, BBoolError> {
        match self.names.get(name) {
            Some(&position) => Ok(self.bools.get_cl_at_pos(position)?),
            None => Err(BBoolError::NotFound(name.to_string())),
        }
    }
    /// Gets a clone of the raw numeric storage.
    ///
    /// # Examples
    /// ```
    /// use btypes::named_bools::BN128;
    /// let bools = BN128::new();
    /// let raw_clone = bools.get_raw_cl();
    /// ```
    pub fn get_raw_cl(&self) -> T {
        self.bools.get_raw_cl()
    }
}

impl<T: BitwiseOpsCopy> IntoIterator for BetterBoolNamed<T>
{
    type Item = (String, bool);
    type IntoIter = std::collections::hash_map::IntoIter<String, bool>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.all().expect("Failed to get all contained bools").into_iter()
    }   
}
