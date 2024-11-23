# btypes

A Rust library providing enhanced types with rich functionality for string manipulation and boolean state management, with more types coming soon.

## Key Features

### BetterString
A feature-rich string type offering:
- Validation for common formats:
  - Email addresses
  - URLs
  - IPv4 addresses
- Pattern matching with regex support
- Encoding/decoding:
  - Base64
  - URL encoding
- String operations:
  - Concatenation using `+` operator
  - Arithmetic operations:
    - `*` repeats a string N times
    - `/` counts occurrences
    - `-` removes a substring
    - `/=` keeps only the first occurrence
  - Case transformations
  - Pattern finding and replacement
  - Safe UTF-8 handling
  - Comprehensive iterator support
  - Rich comparison operations

### Boolean Types
Two main categories with multiple capacity options:

#### Named Boolean Types (Fixed Capacity)
- `BN128`: 128-bit capacity
- `BN64`: 64-bit capacity
- `BN32`: 32-bit capacity
- `BN16`: 16-bit capacity
- `BN8`: 8-bit capacity

#### Infinite Capacity Boolean Types
- `BNInf`: Effectively infinite capacity using dynamic allocation
- Custom types supported via `Nums` and `BitwiseOpsClone`/`BitwiseOpsCopy` traits

Features include:
- Named boolean state management
- Raw binary access and manipulation
- Mass operations with pattern-based flag setting
- Sorting capabilities
- Comprehensive error handling
- Safe and unsafe operation modes
- Iterator support
- Clone and Copy semantics where applicable

## Usage Examples

### String Operations
```rust
use btypes::bstring::BetterString;

fn main() {
    // Basic creation and manipulation
    let mut str1 = BetterString::new("Hello, World!");
    println!("Original: {}", str1);
    println!("Uppercase: {}", str1.to_uppercase());
    
    // Pattern matching and validation
    let email = BetterString::new("user@example.com");
    println!("Valid email: {}", email.is_valid_email());
    
    // Encoding operations
    let encoded = str1.to_base64();
    println!("Base64: {}", encoded);
    
    // Pattern matching and replacement
    let text = BetterString::new("The quick brown fox");
    let matches = text.find_all(r"\w+");
    println!("Words: {:?}", matches);
}
```

### Boolean State Management
```rust
use btypes::named_bools::BN128;

fn main() -> anyhow::Result<()> {
    let mut state = BN128::new();
    
    // Individual state management
    state.set("is_active", true)?;
    state.set("is_validated", false)?;
    
    // Mass operations
    state.mass_set(4, "flag_{n}", "true,false{r}")?;
    
    // State querying
    println!("Active: {}", state.get("is_active")?);
    println!("All states: {:?}", state.all()?);
    
    Ok(())
}
```

### Infinite Boolean Management
```rust
use btypes::inf_named_bools::BNInf;

fn main() -> anyhow::Result<()> {
    let mut state = BNInf::new();
    
    // Can handle very large numbers of flags
    state.mass_set(1000, "flag_{n}", "true,false{r}")?;
    
    // Efficient state management
    println!("States: {:?}", state.all()?);
    
    Ok(())
}
```

## Feature Flags
- `bools`: Basic boolean types
- `named_bools`: Enable named boolean types
- `strings`: String enhancement functionality
- `inf_bools`: Infinite capacity boolean types
- `inf_named_bools`: Named infinite capacity boolean types
- `all`: Enable all features

## Examples
Full working examples available in:
- `/examples/string_ex.rs`: String manipulation demonstrations
- `/examples/bool_ex.rs`: Advanced boolean operations
- `/examples/low_cap_bool_ex.rs`: Different capacity boolean types
- `/examples/inf_cap_bool_ex.rs`: Infinite capacity boolean operations

## License
Licensed under GPL-3.0

## Contributing
Contributions are welcome! Please feel free to submit:
- Pull requests
- Bug reports
- Feature suggestions
- Documentation improvements
