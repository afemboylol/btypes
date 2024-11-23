use btypes::bstring::BetterString;

fn main() {
    // Basic creation and manipulation
    let mut str1 = BetterString::new("Hello, World!");
    println!("Original string: {}", str1);
    println!("Uppercase: {}", str1.to_uppercase());
    println!("Length: {}", str1.len());
    
    // Pattern matching
    let email = BetterString::new("user@example.com");
    println!("Is valid email: {}", email.is_valid_email());
    
    // String operations
    let str2 = BetterString::new(" Additional text");
    str1 += str2;
    println!("Concatenated: {}", str1);
    
    // Encoding
    let encoded = str1.to_base64();
    println!("Base64 encoded: {}", encoded);
    
    // Pattern matching and replacement
    let text = BetterString::new("The quick brown fox jumps over the lazy dog");
    let matches = text.find_all(r"\w+");
    println!("Word matches: {:?}", matches);
}