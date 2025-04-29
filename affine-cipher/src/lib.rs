/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Calculate the greatest common divisor of two numbers
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 { a.abs() } else { gcd(b, a % b) }
}

// Check if two numbers are coprime (their GCD is 1 )
fn is_coprime(a: i32, b: i32) -> bool {
    gcd(a, b) == 1
}

fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    for i in 1..m {
        if (a * i) % m == 1 {
            return Some(i);
        }
    }
    None
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Check if a and m (26) are coprime
    let m = 26;
    if !is_coprime(a, m) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    let mut result = String::new();
    let mut char_count = 0;

    for c in plaintext.chars().filter(|c| c.is_alphanumeric()) {
        if char_count > 0 && char_count % 5 == 0 {
            result.push(' ');
        }

        if c.is_digit(10) {
            result.push(c);
        } else {
            // Convert to lowercase for consistency
            let lowercase_c = c.to_ascii_lowercase();

            // Get the index fo the character (0 - 25 for a-z)
            let i = (lowercase_c as u8 - b'a') as i32;

            // Apply the encryption formula: E(x) = (ax + b) mod m
            let encrypted_index = (a * i + b) % m;

            // Convert back to a character
            let encrypted_char = (encrypted_index as u8 + b'a') as char;

            result.push(encrypted_char);
        }
        char_count += 1;
    }
    Ok(result)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    // Check if a and m (26) are coprime
    let m = 26;
    if !is_coprime(a, m) {
        return Err(AffineCipherError::NotCoprime(a));
    }

    // Find the modular multiplicative inverse of a
    let a_inv = mod_inverse(a, m).unwrap();

    let mut result = String::new();

    for c in ciphertext.chars().filter(|c| c.is_alphanumeric()) {
        if c.is_digit(10) {
            // Digits are not decrypted
            result.push(c);
        } else {
            // Convert to lowercase for consistency
            let lowercase_c = c.to_ascii_lowercase();

            // Get the index of the character (0-25 for a-z)
            let y = (lowercase_c as u8 - b'a') as i32;

            // Apply the decrypted formula: D(y) = a^(-1) * (y - b)  mod m
            // We need to ensure we don't get negative values when subtracting
            let decrypted_index = (a_inv * ((y - b) % m + m)) % m;

            // Convert back to a character
            let decrpyed_char = (decrypted_index as u8 + b'a') as char;

            result.push(decrpyed_char);
        }
    }

    Ok(result)
}
