pub struct Luhn {
    digits: Vec<u32>,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        // Check if we have at least 2 digits
        if self.digits.len() < 2 {
            return false;
        }

        // Check if any non-digit characters were found
        if self.digits.contains(&u32::MAX) {
            return false;
        }

        // Calculate the Luhn sum
        let sum: u32 = self
            .digits
            .iter()
            .rev() // Start from the rightmost digit
            .enumerate()
            .map(|(i, &digit)| {
                // Double every second digit (from the right)
                if i % 2 == 1 {
                    let doubled = digit * 2;
                    // If doubling results in a number > 9, subtract 9
                    if doubled > 9 { doubled - 9 } else { doubled }
                } else {
                    digit
                }
            })
            .sum();

        // The number is valid if the sum is divisible by 10
        sum % 10 == 0
    }
}

// Implementation for string types
impl From<&str> for Luhn {
    fn from(input: &str) -> Self {
        // Convert the input string to a vector of digits,
        // filtering out any whitespace
        let digits: Vec<u32> = input
            .chars()
            .filter(|c| !c.is_whitespace())
            .map(|c| {
                if c.is_digit(10) {
                    c.to_digit(10).unwrap()
                } else {
                    // Mark non-digit characters with a special value
                    u32::MAX
                }
            })
            .collect();

        Luhn { digits }
    }
}

// Implementation for String
impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Luhn::from(input.as_str())
    }
}

// Implementation for u8
impl From<u8> for Luhn {
    fn from(num: u8) -> Self {
        digits_from_number(num as u64)
    }
}

// Implementation for u16
impl From<u16> for Luhn {
    fn from(num: u16) -> Self {
        digits_from_number(num as u64)
    }
}

// Implementation for u32
impl From<u32> for Luhn {
    fn from(num: u32) -> Self {
        digits_from_number(num as u64)
    }
}

// Implementation for u64
impl From<u64> for Luhn {
    fn from(num: u64) -> Self {
        digits_from_number(num)
    }
}

// Implementation for usize
impl From<usize> for Luhn {
    fn from(num: usize) -> Self {
        digits_from_number(num as u64)
    }
}

// Helper function to extract digits from a number
fn digits_from_number(mut num: u64) -> Luhn {
    let mut digits = Vec::new();

    if num == 0 {
        digits.push(0);
    } else {
        while num > 0 {
            digits.push((num % 10) as u32);
            num /= 10;
        }
        digits.reverse(); // We extracted digits from right to left, so reverse them
    }

    Luhn { digits }
}
