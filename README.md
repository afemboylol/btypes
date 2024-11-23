# btypes

A Rust library providing enhanced types with additional features and functionality.

## Features

- **BetterString**: An enhanced string type with additional operations and utility methods
  - String validation (email, URL, IPv4)
  - Pattern matching and replacement
  - Base64 and URL encoding/decoding
  - Arithmetic operations (concatenation, multiplication)
  - Advanced string manipulation methods

- **Boolean Types**
  - Advanced boolean operations
  - Named boolean states
  - State management capabilities
  - Raw binary access

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
btypes = "0.2.2"
```

## Feature Flags

The library supports the following feature flags:

- `named_bools`: Enables named boolean functionality (includes `bools`)
- `bools`: Basic boolean functionality
- `strings`: String enhancement functionality
- `all`: Enables all features

## Examples

### BetterString Usage

```rust
use btypes::bstring::BetterString;

fn main() {
    // Basic string operations
    let mut stri = BetterString::new("Hello, World!");
    println!("Original: {}", stri);
    println!("Uppercase: {}", stri.to_uppercase());

    // Validation
    let email = BetterString::new("user@example.com");
    println!("Is valid email: {}", email.is_valid_email());

    // Encoding
    let encoded = stri.to_base64();
    println!("Base64 encoded: {}", encoded);
}
```

### Boolean Operations

See the examples directory for detailed boolean operation examples:
- `examples/bool_ex.rs`
- `examples/low_cap_bool_ex.rs`

## License

This project is licensed under the GPL-3.0 License.
