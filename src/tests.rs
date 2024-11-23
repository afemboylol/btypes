#[cfg(test)]
mod tests {
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
        let mut bools = BN128::new();

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
