use rand::Rng;

fn is_valid_key(key: &str) -> bool {
    if key.is_empty() {
        return false;
    }
    for c in key.chars() {
        if !c.is_ascii_lowercase() {
            return false;
        }
    }
    true
}
pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) || s.is_empty() {
        return None; // Handle empty input strings
    }

    let mut result = String::with_capacity(s.len()); // Pre-allocate for efficiency

    for (i, char_s) in s.char_indices() {
        // Iterate with character indices
        let key_char = key
            .chars()
            .nth(i % key.len()) // Cycle through the key
            .expect("Key should always have a character at this index"); // Safe because of modulo operation.  Could potentially handle the empty key case differently.

        let char_code_s = (char_s as u8 - b'a') as u32; // Gets 0 for 'a', 1 for 'b', etc.
        let key_code = char_code(key_char); // Convert to u32

        // Perform the addition and handle wrapping (crucial for correct encoding).
        let sum = (char_code_s + key_code) % 26;

        //Convert back to a character
        let encoded_char = (sum as u8 + b'a') as char;

        result.push(encoded_char);
    }

    Some(result)
}

fn char_code(c: char) -> u32 {
    (c as u8 - b'a') as u32
}

pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) || s.is_empty() {
        return None; // Handle empty input strings
    }

    let mut result = String::with_capacity(s.len());

    for (i, char_s) in s.char_indices() {
        let key_char = key
            .chars()
            .nth(i % key.len())
            .expect("Key should always have a character at this index");

        let char_code_s = (char_s as u8 - b'a') as u32; // Get alphabet position
        let key_code = char_code(key_char); // Get alphabet position

        let sum = (char_code_s + 26 - key_code) % 26; // Wrap within alphabet (subtract, then wrap)

        let encoded_char = (sum as u8 + b'a') as char; // Convert back to char

        result.push(encoded_char);
    }

    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::thread_rng();
    let key_length = rng.gen_range(100..=200); // Key length between 100 and 200
    let key: String = (0..key_length)
        .map(|_| (b'a' + rng.gen_range(0..26)) as char)
        .collect();

    let encoded_s = encode(&key, s).expect("Encoding should always succeed with a valid key"); //encode function requires an option

    (key, encoded_s)
}
