# btypes

A Rust library providing enhanced types with rich functionality for string manipulation and boolean state management, eventually more.

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
  - Arithmetic operations (* repeats a string N times, / counts occurences, and - removes a substring. /= keeps only the first occurence.)
  - Case transformations
  - Pattern finding and replacement

### Named Boolean Types
Multiple capacity options for different use cases:
- `BN128`: 128-bit capacity
- `BN64`: 64-bit capacity
- `BN32`: 32-bit capacity
- `BN16`: 16-bit capacity
- `BN8`: 8-bit capacity
- Any custom type which implements Nums and either BitwiseOpsClone or BitwiseOpsCopy.

Features include:
- Named boolean state management
- Raw binary access and manipulation
- Bulk operations and pattern-based flag setting
- Sorting capabilities
- Capacity management and error handling

## Coming Soon
- Any user suggestions I am capable of
- More string operations and utilities
- More bool operations and utilities (especially for the infinite bools)

## Usage Examples

### String Operations
```rust
use btypes::bstring::BetterString;

fn main() {
    // Basic string manipulation
    let mut text = BetterString::new("Hello, World!");
    println!("Original: {}", text);
    
    // Pattern matching
    let email = BetterString::new("user@example.com");
    println!("Valid email: {}", email.is_valid_email());
    
    // Encoding
    println!("Base64: {}", text.to_base64());
}
```

### Boolean State Management
```rust
use btypes::named_bools::BN128;

fn main() -> anyhow::Result<()> {
    let mut state = BN128::new();
    
    // State management
    state.set("is_active", true)?;
    state.set("is_validated", false)?;
    
    // Binary operations
    println!("Raw state: {:b}", state.get_raw());
    Ok(())
}
```

## Feature Flags
- `named_bools`: Enable named boolean types (includes `bools`)
- `bools`: Basic boolean operations
- `strings`: String enhancement functionality
- `all`: Enable all features

## Examples
Full working examples available in:
- `/examples/string_ex.rs`: String manipulation demos
- `/examples/bool_ex.rs`: Advanced boolean operations
- `/examples/low_cap_bool_ex.rs`: Different capacity boolean types

## License
Licensed under GPL-3.0

## Contributing
Contributions are welcome! Please feel free to submit pull requests, bug reports, or feature suggestions.
