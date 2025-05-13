pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

// Helper function for the Luhn algorithm on integers
fn valid_luhn_for_integer(num: u64) -> bool {
    if num == 0 {
        return true; // 0 is considered valid in Luhn algorithm
    }

    let mut num = num;
    let mut sum = 0;
    let mut count = 0;
    let mut double_next = false;

    while num > 0 {
        let digit = (num % 10) as u32;
        num /= 10;

        count += 1;

        if double_next {
            let doubled = digit * 2;
            sum += if doubled > 9 { doubled - 9 } else { doubled };
        } else {
            sum += digit;
        }

        double_next = !double_next;
    }

    // Luhn check requires at least 2 digits
    count >= 2 && sum % 10 == 0
}

// Implement for each integer type individually
impl Luhn for u8 {
    fn valid_luhn(&self) -> bool {
        valid_luhn_for_integer(*self as u64)
    }
}

impl Luhn for u16 {
    fn valid_luhn(&self) -> bool {
        valid_luhn_for_integer(*self as u64)
    }
}

impl Luhn for u32 {
    fn valid_luhn(&self) -> bool {
        valid_luhn_for_integer(*self as u64)
    }
}

impl Luhn for u64 {
    fn valid_luhn(&self) -> bool {
        valid_luhn_for_integer(*self)
    }
}

impl Luhn for usize {
    fn valid_luhn(&self) -> bool {
        valid_luhn_for_integer(*self as u64)
    }
}

impl Luhn for &str {
    fn valid_luhn(&self) -> bool {
        // Filter out spaces and non-digit characters
        let digits: Vec<char> = self.chars().filter(|c| c.is_digit(10)).collect();

        // Luhn check requires at least 2 digits
        if digits.len() < 2 {
            return false;
        }

        let mut sum = 0;
        let mut double_next = false;

        // Process digits from right to left
        for &digit_char in digits.iter().rev() {
            let digit = digit_char.to_digit(10).unwrap();

            if double_next {
                let doubled = digit * 2;
                sum += if doubled > 9 { doubled - 9 } else { doubled };
            } else {
                sum += digit;
            }

            double_next = !double_next;
        }

        sum % 10 == 0
    }
}

impl Luhn for String {
    fn valid_luhn(&self) -> bool {
        self.as_str().valid_luhn()
    }
}
