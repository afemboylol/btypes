use btypes::named_bools::BN128;
fn main() -> anyhow::Result<()> {
    // Example 1: Basic Operations with Binary Initialization
    println!("=== Example 1: Basic Operations ===");
    let mut bools = BN128::from_num(0b10101010);
    println!("Initial binary state: {:b}", bools.get_raw());
    println!("Initial mapped state: {:#?}", bools.all()?);

    // Adding and modifying named bools
    bools.set("flag_a", true)?;
    bools.set("flag_b", false)?;
    bools.set("flag_c", true)?;
    println!("After adding flags: {:#?}", bools.all()?);

    // Example 2: Advanced Operations
    println!("\n=== Example 2: Advanced Operations ===");
    let mut advanced = BN128::new();

    // Mass setting flags
    advanced.mass_set(27, "test_{n}", "true,false{r}").expect("Failed to set flags");
    println!("Mass-set flags: {:#?}", advanced.all()?);

    // Sorting
    advanced.sort()?;
    println!("Sorted flags: {:#?}", advanced.all()?);
    
    // Bulk operations with different patterns
    for i in 0..8 {
        // Alternating pattern
        advanced.set(&format!("alt_{}", i), i % 2 == 0)?;
        // Every third flag
        advanced.set(&format!("third_{}", i), i % 3 == 0)?;
    }
    println!("Complex pattern flags: {:#?}", advanced.all()?);
    
    // Demonstrate raw access and manipulation
    println!("Raw binary state: {:b}", advanced.get_raw());
    
    // Example 3: State Management
    println!("\n=== Example 3: State Management ===");
    let mut state = BN128::new();
    
    // Setting up state flags
    state.set("is_active", true)?;
    state.set("is_validated", false)?;
    state.set("has_error", false)?;
    
    // Check and modify state
    if state.get("is_active")? {
        state.set("is_validated", true)?;
    }
    
    println!("Current state: {:#?}", state.all()?);
    println!("is_active exists: {}", state.exists("is_active"));
    println!("unknown exists: {}", state.exists("unknown"));

    // Example 4: Capacity and Error Handling
    println!("\n=== Example 4: Capacity and Error Handling ===");
    let mut capacity_test = BN128::new();
    
    // Fill up to capacity
    for i in 0..128 {
        capacity_test.set(&format!("flag_{}", i), true)?;
    }
    println!("Capacity count: {}", capacity_test.all_names().len());
    
    // Try to exceed capacity (should return error)
    match capacity_test.set("overflow", true) {
        Ok(_) => println!("Added successfully (unexpected)"),
        Err(e) => println!("Expected error: {}", e),
    }

    Ok(())
}
