#[cfg(test)]
mod bool_tests {
    use crate::bbool::B128;
    use crate::named_bools::BN128;

    #[test]
    fn test_basic_operations() {
        let mut bools = BN128::from_num(0);
        assert!(bools.set("test1", true).is_ok());
        assert!(bools.get("test1").unwrap());

        assert!(bools.set("test2", false).is_ok());
        assert!(!bools.get("test2").unwrap());
    }

    #[test]
    fn test_from_num() {
        let bools = BN128::from_num(0b101);
        assert_eq!(bools.get_raw_cl(), 0b101);
        assert_eq!(*bools.get_raw(), 0b101);
    }

    #[test]
    fn test_capacity() {
        let mut bools = BN128::from_num(0);
        for i in 0..128 {
            assert!(bools.set(&format!("bool_{}", i), true).is_ok());
        }
        // Should fail on 129th addition
        assert!(bools.set("overflow", true).is_err());
    }

    #[test]
    fn test_exists() {
        let mut bools = BN128::new();
        assert!(bools.set("test_bool", true).is_ok());
        assert!(bools.exists("test_bool"));
        assert!(!bools.exists("nonexistent"));
    }

    #[test]
    fn test_all_functions() {
        let mut bools = BN128::new();

        // Set up some test data
        assert!(bools.set("bool1", true).is_ok());
        assert!(bools.set("bool2", false).is_ok());
        assert!(bools.set("bool3", true).is_ok());

        // Test all()
        let all_map = bools.all().unwrap();
        assert!(all_map.get("bool1").unwrap());
        assert!(!all_map.get("bool2").unwrap());
        assert!(all_map.get("bool3").unwrap());

        // Test all_names()
        let names = bools.all_names();
        assert!(names.contains_key("bool1"));
        assert!(names.contains_key("bool2"));
        assert!(names.contains_key("bool3"));

        // Test all_names_cl()
        let names_clone = bools.all_names_cl();
        assert_eq!(names, &names_clone);
    }

    #[test]
    fn test_error_handling() {
        let bools = BN128::new();

        // Test getting nonexistent bool
        assert!(bools.get("nonexistent").is_err());

        // Test getting with clone for nonexistent bool
        assert!(bools.get_cl("nonexistent").is_err());
    }

    #[test]
    fn test_modification() {
        let mut bools = BN128::new();

        // Test adding and modifying
        assert!(bools.set("test_bool", true).is_ok());
        assert!(bools.get("test_bool").unwrap());

        assert!(bools.set("test_bool", false).is_ok());
        assert!(!bools.get("test_bool").unwrap());
    }

    #[test]
    fn test_raw_operations() {
        let mut bools = BN128::from_num(0);

        // Test raw value modifications
        assert!(bools.set("bit0", true).is_ok()); // Sets first bit
        assert_eq!(*bools.get_raw(), 1); // Binary: ...0001

        assert!(bools.set("bit1", true).is_ok()); // Sets second bit
        assert_eq!(*bools.get_raw(), 3); // Binary: ...0011

        assert!(bools.set("bit0", false).is_ok()); // Clears first bit
        assert_eq!(*bools.get_raw(), 2); // Binary: ...0010
    }
    #[test]
    fn test_inc_unchecked_normal_operation() {
        // Create a B128 instance with 128 cap
        let mut bool = B128::from_num(8);
        assert_eq!(bool.reader_head_pos, 0);

        // Safe usage within bounds
        unsafe {
            bool.inc_unchecked();
        }
        assert_eq!(bool.reader_head_pos, 1);
    }

    #[test]
    fn test_inc_unchecked_multiple_increments() {
        let mut bool = B128::new();

        // Multiple safe increments
        unsafe {
            for _ in 0..5 {
                bool.inc_unchecked();
            }
        }
        assert_eq!(bool.reader_head_pos, 5);
    }

    // This test documents how the inc_unchecked method should be used safely
    #[test]
    fn test_inc_unchecked_with_bounds_check() {
        let mut bool = B128::new();

        // Example of safe usage pattern
        unsafe {
            if bool.reader_head_pos < B128::CAP - 1 {
                bool.inc_unchecked();
            }
        }
        assert_eq!(bool.reader_head_pos, 1);
    }

    // This test documents how the inc_unchecked method should not be used
    #[test]
    fn test_inc_unchecked_unsafe() {
        let mut bool = B128::new();

        bool.shp(127).unwrap();

        assert_eq!(bool.reader_head_pos, 127);
        unsafe {
            bool.inc_unchecked();
        }

        assert_eq!(bool.reader_head_pos, 128);
    }
}

/// Example usage and tests for BetterString
#[cfg(test)]
mod string_tests {
    use crate::bstring::BetterString;
    use std::str::FromStr;

    #[test]
    fn test_basic_operations() {
        let str1 = BetterString::new("Hello");
        assert_eq!(str1.len(), 5);
        assert!(!str1.is_empty());
        assert_eq!(str1.to_uppercase(), BetterString::from("HELLO"));
        assert_eq!(str1.to_lowercase(), BetterString::from("hello"));
    }    

    #[test]
    fn test_string_validation() {
        let email = BetterString::new("test@example.com");
        let url = BetterString::new("https://www.example.com");
        let ipv4 = BetterString::new("192.168.1.1");

        assert!(email.is_valid_email());
        assert!(url.is_valid_url());
        assert!(ipv4.is_valid_ipv4());
    }

    #[test]
    fn test_pattern_matching() {
        let text = BetterString::new("Hello, World! Hello");
        assert!(text.matches_pattern(r"^Hello"));

        let matches = text.find_all("Hello");
        assert_eq!(matches.len(), 2);

        let replaced = text.replace_all("Hello", "Hi");
        assert_eq!(replaced.to_string(), "Hi, World! Hi");
    }

    #[test]
    fn test_encoding() {
        let original = BetterString::new("Test String");
        let encoded = original.to_base64();
        let decoded = BetterString::from_base64(&encoded).unwrap();
        assert_eq!(original, decoded);

        let url_text = BetterString::new("Hello World!");
        let url_encoded = url_text.to_url_encoded();
        let url_decoded = BetterString::from_url_encoded(&url_encoded).unwrap();
        assert_eq!(url_text, url_decoded);
    }

    #[test]
    fn test_arithmetic_operations() {
        let str1 = BetterString::new("Hello");
        let str2 = BetterString::new(" World");

        // Addition
        let combined = str1.clone() + str2.clone();
        assert_eq!(combined.to_string(), "Hello World");

        // Multiplication
        let repeated = str1.clone() * 3;
        assert_eq!(repeated.to_string(), "HelloHelloHello");

        // Subtraction
        let str3 = BetterString::new("Hello World");
        let str4 = BetterString::new("World");
        let subtracted = str3 - str4;
        assert_eq!(subtracted.to_string(), "Hello ");
    }

    #[test]
    fn test_utility_methods() {
        let text = BetterString::new("  Hello World  ");
        assert_eq!(text.trim(), BetterString::from("Hello World"));

        let words: Vec<BetterString> = text
            .split(" ")
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect();
        assert_eq!(words, vec!["Hello", "World"]);

        let palindrome = BetterString::new("A man a plan a canal Panama");
        assert!(palindrome.is_palindrome());
    }

    #[test]
    fn test_string_properties() {
        let numeric = BetterString::new("12345");
        let alpha = BetterString::new("abcde");
        let alphanum = BetterString::new("abc123");
        let whitespace = BetterString::new("   ");

        assert!(numeric.is_numeric());
        assert!(alpha.is_alphabetic());
        assert!(alphanum.is_alphanumeric());
        assert!(whitespace.is_whitespace());
    }

    #[test]
    fn test_error_handling() {
        let empty = BetterString::new("");
        assert!(empty.safe_split(",").is_err());

        let invalid_substring = BetterString::new("test");
        assert!(invalid_substring.substring(5, 10).is_err());
    }

    #[test]
    fn test_conversion_traits() {
        // From String
        let string = String::from("test");
        let bstring: BetterString = string.clone().into();
        assert_eq!(bstring.to_string(), string);

        // From &str
        let bstring_from_str: BetterString = "test".into();
        assert_eq!(bstring_from_str.to_string(), "test");

        // FromStr
        let parsed = BetterString::from_str("test").unwrap();
        assert_eq!(parsed.to_string(), "test");
    }

    #[test]
    fn test_iterator_support() {
        let bstring = BetterString::new("abc");
        // Test owned iteration
        let bytes: Vec<u8> = bstring.clone().into_iter().collect();
        assert_eq!(bytes, vec![b'a', b'b', b'c']);

        // Test reference iteration - corrected version
        let byte_refs: Vec<&u8> = (&bstring).into_iter().collect();
        assert_eq!(byte_refs, vec![&b'a', &b'b', &b'c']);
    }
}

#[cfg(test)]
mod inf_named_bools_tests {
    use crate::error::BBoolError;
    use crate::inf_named_bools::BNInf;
    use anyhow::Result;

    #[test]
    fn test_new_and_default() {
        let bool = BNInf::new();
        assert!(bool.bools.store.is_empty());
        assert!(bool.all_names().is_empty());
    }

    #[test]
    fn test_from_vec() {
        let initial = vec![5u8]; // Binary: 00000101
        let bool = BNInf::from_vec(initial.clone());
        assert_eq!(bool.bools.store, initial);
        assert!(bool.all_names().is_empty());
    }

    #[test]
    fn test_get_set_operations() -> Result<(), BBoolError> {
        let mut bool = BNInf::new();

        // Test setting and getting named values
        bool.set("test1", true)?;
        assert!(bool.get("test1")?);

        bool.set("test2", false)?;
        assert!(!bool.get("test2")?);

        // Test updating existing value
        bool.set("test1", false)?;
        assert!(!bool.get("test1")?);

        Ok(())
    }

    #[test]
    fn test_mass_operations() -> Result<()> {
        let mut bool = BNInf::new();

        // Test mass_set
        bool.mass_set(3, "test_{n}", "true,false{r}")?;
        assert!(bool.get("test_0")?);
        assert!(!bool.get("test_1")?);
        assert!(bool.get("test_2")?);

        // Test mass_get
        let values = bool.mass_get(&["test_0", "test_1", "test_2"])?;
        assert_eq!(values, vec![true, false, true]);

        // Test mass_toggle
        bool.mass_toggle(&["test_0", "test_1"])?;
        assert!(!bool.get("test_0")?);
        assert!(bool.get("test_1")?);

        Ok(())
    }

    #[test]
    fn test_all_and_sorted() -> Result<()> {
        let mut bool = BNInf::new();

        // Set up test data
        bool.set("c", true)?;
        bool.set("a", false)?;
        bool.set("b", true)?;

        // Test all()
        let all_pairs = bool.all()?;
        assert_eq!(all_pairs.len(), 3);
        assert!(!all_pairs["a"]);
        assert!(all_pairs["b"]);
        assert!(all_pairs["c"]);

        // Test sorted()
        let sorted = bool.sorted()?;
        let sorted_pairs = sorted.all()?;
        let mut keys: Vec<_> = sorted_pairs.keys().collect();
        keys.sort();
        assert_eq!(keys, vec!["a", "b", "c"]);

        Ok(())
    }

    #[test]
    fn test_exists_and_delete() -> Result<()> {
        let mut bool = BNInf::new();

        bool.set("test", true)?;
        assert!(bool.exists("test"));

        bool.delete("test")?;
        assert!(!bool.exists("test"));

        Ok(())
    }

    #[test]
    fn test_raw_access() {
        let mut bool = BNInf::from_vec(vec![5]); // Binary: 00000101

        assert_eq!(bool.get_raw(), &vec![5]);

        let raw_mut = bool.get_raw_mut();
        raw_mut[0] = 3; // Binary: 00000011
        assert_eq!(bool.get_raw(), &vec![3]);
    }

    #[test]
    fn test_cap() -> Result<()> {
        let mut bool = BNInf::with_cap(8);
        bool.set("test1", true)?;
        bool.set("test2", true)?;

        assert!(bool.cap() == 8);

        let mut bool2 = BNInf::with_cap(9);
        bool2.set("test1", true)?;
        bool2.set("test2", true)?;

        assert!(bool.cap() == 8);

        Ok(())
    }

    #[test]
    fn test_clear() -> Result<()> {
        let mut bool = BNInf::new();
        bool.set("test1", true)?;
        bool.set("test2", true)?;

        bool.clear();
        assert!(bool.all_names().is_empty());
        assert!(bool.bools.store.is_empty());

        Ok(())
    }

    #[test]
    fn test_error_conditions() -> Result<()> {
        let mut bool = BNInf::new();

        // Test getting non-existent name
        assert!(bool.get("nonexistent").is_err());

        // Test invalid pattern in mass_set
        assert!(bool.mass_set(3, "test", "true,false{r}").is_err()); // Missing {n}
        assert!(bool.mass_set(3, "test_{n}", "true").is_err()); // Insufficient values

        Ok(())
    }
}
