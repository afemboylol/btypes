use crate::error::BBoolError;
use crate::inf_bbool::BetterBoolInf;
use anyhow::Error;
use anyhow::Result;
use std::{collections::HashMap, marker::PhantomData};

/// Type alias for the infinite-capacity named boolean collection
pub type BNInf = BetterBoolNamedInf;

/// A dynamically-sized collection of named boolean values
///
/// This struct combines the unlimited capacity of `BetterBoolInf` with
/// the ability to access boolean values by name rather than position.
#[derive(Default)]
pub struct BetterBoolNamedInf {
    /// The underlying boolean storage
    pub bools: BetterBoolInf,
    /// Mapping of names to boolean positions
    names: HashMap<String, u128>,
    /// Next available position for new boolean values
    _next_assign: u128,
}

impl BetterBoolNamedInf {
    /// Creates a new `BetterBoolNamedInf` instance with a specified initial vector value.
    ///
    /// # Arguments
    /// * `initial_value` - The initial vector to store the boolean states
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let bools = BNInf::from_vec(vec![0b01010110]);
    /// ```
    ///
    #[must_use] pub fn from_vec(initial_value: Vec<u8>) -> Self {
        let bools = BetterBoolInf {
            store: initial_value,
            reader_head_pos: 0,
            _marker: PhantomData,
        };
        Self {
            bools,
            names: HashMap::new(),
            _next_assign: 0,
        }
    }

    /// Creates a new empty `BetterBoolNamedInf` instance initialized with an empty vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let bools = BNInf::new();
    /// ```
    ///
    #[must_use] pub fn new() -> Self {
        Default::default()
    }

    /// Set/add many named bools, with the names being dictated by the pattern and the values by the value pattern.
    ///
    /// # Arguments
    /// * count - Number of bools to set/add
    /// * pattern - Name pattern containing {n} which will be replaced with sequential numbers (0 to count-1)
    /// * `value_pattern` - Comma-separated list of boolean values with optional {r} suffix to repeat the pattern (if list length does not contain {r}, or exceed)
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// fn main() -> anyhow::Result<()> {
    /// let mut bools = BNInf::new();
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
        count: u128,
        pattern: &str,
        value_pattern: &str,
    ) -> Result<(), BBoolError> {
        if !pattern.contains("{n}") {
            return Err(BBoolError::InvalidPattern(
                "Pattern must contain {n}".to_string(),
            ));
        }

        let value_parts: Vec<&str> = value_pattern.trim().split(',').collect();
        if value_parts.is_empty() {
            return Err(BBoolError::InvalidPattern(
                "Value pattern cannot be empty".to_string(),
            ));
        }
        if !value_pattern.contains("{r}") && value_parts.len() < count as usize {
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

        for i in 0..count {
            let name = pattern.replace("{n}", &i.to_string());
            let value_index = if repeating {
                (i as usize) % values.len()
            } else {
                if i as usize >= values.len() {
                    let last = values.last().unwrap();
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
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
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
    pub fn mass_get(&mut self, names: &[&str]) -> Result<Vec<bool>> {
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
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
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
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
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

    /// Returns a new `BetterBoolNamedInf` instance with contents sorted by name.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("c", true)?;
    /// bools.add("a", false)?;
    /// let sorted_bools = bools.sorted()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the sorting operation fails
    pub fn sorted(&mut self) -> Result<Self> {
        let mut pairs: Vec<_> = self.all()?.into_iter().collect();
        pairs.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut sorted = Self::new();

        for (name, value) in pairs {
            sorted.add(&name, value)?;
        }

        Ok(sorted)
    }

    /// Returns all boolean values in the collection as a vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test1", true)?;
    /// bools.add("test2", false)?;
    /// let all_values = bools.all_bools()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if retrieving any boolean value fails
    pub fn all_bools(&mut self) -> Result<Vec<bool>> {
        self.bools.all()
    }

    /// Returns a clone of the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let bools = BNInf::new();
    /// let names = bools.all_names_cl();
    /// ```
    ///
    #[must_use] pub fn all_names_cl(&self) -> HashMap<String, u128> {
        self.names.clone()
    }

    /// Returns a reference to the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let bools = BNInf::new();
    /// let names = bools.all_names();
    /// ```
    ///
    #[must_use] pub const fn all_names(&self) -> &HashMap<String, u128> {
        &self.names
    }

    /// Returns a mutable reference to the internal name-to-position mapping.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let mut bools = BNInf::new();
    /// let names_mut = bools.all_names_mut();
    /// ```
    ///
    pub fn all_names_mut(&mut self) -> &mut HashMap<String, u128> {
        &mut self.names
    }

    /// Returns all name-value pairs in the collection.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test1", true)?;
    /// let all_pairs = bools.all()?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if retrieving any boolean value fails
    pub fn all(&mut self) -> Result<HashMap<String, bool>> {
        let mut result = HashMap::new();
        for (name, &position) in &self.names {
            result.insert(name.clone(), self.bools.get_at_pos(position)?);
        }
        Ok(result)
    }

    /// Sets a boolean value for the given name. Creates a new entry if the name doesn't exist.
    ///
    /// # Arguments
    /// * name - The name of the boolean value
    /// * value - The boolean value to set
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.set("test", true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if setting the value fails
    pub fn set(&mut self, name: &str, value: bool) -> Result<()> {
        match self.names.get(name) {
            Some(&position) => self.bools.set_at_pos(position, value)?,
            None => self.add(name, value)?,
        }
        Ok(())
    }

    /// Toggles the boolean value for the given name.
    ///
    /// # Arguments
    /// * name - The name of the boolean value to toggle
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test", true)?;
    /// bools.toggle("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the name doesn't exist or toggling fails
    pub fn toggle(&mut self, name: &str) -> Result<()> {
        let current = self.get(name)?;
        self.set(name, !current)?;
        Ok(())
    }

    /// Checks if a boolean value with the given name exists.
    ///
    /// # Arguments
    /// * name - The name to check
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let mut bools = BNInf::new();
    /// let exists = bools.exists("test");
    /// ```
    ///
    pub fn exists(&mut self, name: &str) -> bool {
        self.names.contains_key(name)
    }

    /// Returns a reference to the raw underlying vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let bools = BNInf::new();
    /// let raw = bools.get_raw();
    /// ```
    ///
    #[must_use] pub const fn get_raw(&self) -> &Vec<u8> {
        self.bools.get_raw()
    }

    /// Returns a mutable reference to the raw underlying vector.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let mut bools = BNInf::new();
    /// let raw_mut = bools.get_raw_mut();
    /// ```
    ///
    pub fn get_raw_mut(&mut self) -> &mut Vec<u8> {
        self.bools.get_raw_mut()
    }

    /// Adds a new named boolean value to the collection.
    ///
    /// # Arguments
    /// * name - The name for the new boolean value
    /// * value - The boolean value to add
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test", true)?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if:
    /// * The collection capacity is reached
    /// * Setting the value fails
    pub fn add(&mut self, name: &str, value: bool) -> Result<(), BBoolError> {
        if self.names.len() >= u128::MAX as usize {
            return Err(BBoolError::CollectionCapacityReached);
        }
        self.names.insert(name.to_string(), self._next_assign);
        self.bools.set_at_pos(self._next_assign, value)?;
        self._next_assign += 1;
        Ok(())
    }

    /// Gets the boolean value associated with the given name.
    ///
    /// # Arguments
    /// * name - The name of the boolean value to retrieve
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test", true)?;
    /// let value = bools.get("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if the name doesn't exist
    pub fn get(&mut self, name: &str) -> Result<bool, BBoolError> {
        match self.names.get(name) {
            Some(&position) => Ok(self.bools.get_at_pos(position)?),
            None => Err(BBoolError::NotFound(name.to_string())),
        }
    }

    /// Deletes a named boolean value from the collection.
    ///
    /// # Arguments
    /// * name - The name of the boolean value to delete
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// use anyhow::Result;
    /// fn main() -> Result<()> {
    /// let mut bools = BNInf::new();
    /// bools.add("test", true)?;
    /// bools.delete("test")?;
    /// Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// Returns an error if setting the value to false fails
    pub fn delete(&mut self, name: &str) -> Result<()> {
        if self.names.contains_key(name) {
            self.set(name, false)?;
            self.names.remove(name);
        }
        Ok(())
    }

    /// Clears all named boolean values from the collection.
    ///
    /// # Examples
    /// ```
    /// use btypes::inf_named_bools::BNInf;
    /// let mut bools = BNInf::new();
    /// bools.clear();
    /// ```
    ///
    pub fn clear(&mut self) {
        self.names.clear();
        self.bools.clear();
    }
}
