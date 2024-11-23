use btypes::inf_named_bools::BNInf;
use anyhow::Result;

fn main() -> Result<()> {
    // Example 1: Basic Operations
    println!("=== Example 1: Basic Operations ===");
    let mut inf_bools = BNInf::new();
    
    // Adding individual flags
    inf_bools.set("flag_a", true)?;
    inf_bools.set("flag_b", false)?;
    inf_bools.set("flag_c", true)?;
    println!("Initial state: {:#?}", inf_bools.all()?);

    // Example 2: Mass Operations
    println!("\n=== Example 2: Mass Operations ===");
    
    // Setting many flags at once with a pattern
    inf_bools.mass_set(1000, "flag_{n}", "true,false{r}")?;
    println!("Number of flags after mass set: {}", inf_bools.all_names().len());
    
    // Mass toggle example
    let flags_to_toggle = ["flag_1", "flag_2", "flag_3"];
    inf_bools.mass_toggle(&flags_to_toggle)?;
    
    // Mass get example
    let values = inf_bools.mass_get(&flags_to_toggle)?;
    println!("Mass get results: {:?}", values);

    // Example 3: Large Scale Operations
    println!("\n=== Example 3: Large Scale Operations ===");
    let mut large_bools = BNInf::new();
    
    // Create a large number of flags
    large_bools.mass_set(10000, "large_{n}", "true{r}")?;
    println!("Created 10000 flags");
    
    // Demonstrate sorting capability
    large_bools.sort()?;
    println!("Sorted all flags");

    // Example 4: State Management
    println!("\n=== Example 4: State Management ===");
    let mut state = BNInf::new();
    
    // Setting up complex state flags
    state.set("system.initialized", true)?;
    state.set("system.ready", false)?;
    state.set("user.authenticated", false)?;
    
    // Toggle some states
    state.toggle("system.ready")?;
    
    println!("Final state: {:#?}", state.all()?);
    
    // Example 5: Cleanup and Memory Management
    println!("\n=== Example 5: Cleanup Operations ===");
    
    // Delete specific flags
    state.delete("system.initialized")?;
    println!("After deletion: {:#?}", state.all()?);
    
    // Clear all flags
    state.clear();
    println!("After clearing all flags: {:#?}", state.all()?);

    Ok(())
}
