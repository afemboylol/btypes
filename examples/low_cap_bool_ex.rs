use btypes::named_bools::{BN64, BN32, BN16, BN8};

fn main() -> anyhow::Result<()> {
    // BN64 Example
    println!("\n=== BN64 Example ===");
    let mut bools64 = BN64::from_num(0b10101010);
    println!("Initial binary state: {:b}", bools64.get_raw());
    println!("Initial mapped state: {:#?}", bools64.all()?);
    
    // Fill up to capacity (64 bits)
    for i in 0..64 {
        bools64.set(&format!("flag_{}", i), i % 2 == 0)?;
    }
    println!("Capacity count BN64: {}", bools64.all_names().len());

    // BN32 Example
    println!("\n=== BN32 Example ===");
    let mut bools32 = BN32::from_num(0b10101010);
    println!("Initial binary state: {:b}", bools32.get_raw());
    println!("Initial mapped state: {:#?}", bools32.all()?);
    
    // Fill up to capacity (32 bits)
    for i in 0..32 {
        bools32.set(&format!("flag_{}", i), i % 2 == 0)?;
    }
    println!("Capacity count BN32: {}", bools32.all_names().len());

    // BN16 Example
    println!("\n=== BN16 Example ===");
    let mut bools16 = BN16::from_num(0b1010);
    println!("Initial binary state: {:b}", bools16.get_raw());
    println!("Initial mapped state: {:#?}", bools16.all()?);
    
    // Fill up to capacity (16 bits)
    for i in 0..16 {
        bools16.set(&format!("flag_{}", i), i % 2 == 0)?;
    }
    println!("Capacity count BN16: {}", bools16.all_names().len());

    // BN8 Example
    println!("\n=== BN8 Example ===");
    let mut bools8 = BN8::from_num(0b10101010);
    println!("Initial binary state: {:b}", bools8.get_raw());
    println!("Initial mapped state: {:#?}", bools8.all()?);

    // State management example with BN8
    bools8.set("active", true)?;
    bools8.set("valid", false)?;
    bools8.set("error", true)?;
    println!("BN8 state: {:#?}", bools8.all()?);

    // Error handling example (trying to exceed capacity)
    println!("\n=== Error Handling Examples ===");
    let mut capacity_test8 = BN8::new();
    
    // Fill BN8 to capacity
    for i in 0..8 {
        capacity_test8.set(&format!("f{}", i), true)?;
    }

    // Try to exceed capacity for each type
    match capacity_test8.set("overflow", true) {
        Ok(_) => println!("BN8: Added successfully (unexpected)"),
        Err(e) => println!("BN8: Expected error: {}", e),
    }

    Ok(())
}
