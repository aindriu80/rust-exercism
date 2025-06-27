use std::cmp::Ordering;
use std::ops::{Add, Mul, Sub};

/// Type implementing arbitrary-precision decimal arithmetic
#[derive(Debug, Clone)]
pub struct Decimal {
    /// The digits of the number (both integer and fractional parts)
    digits: Vec<u8>,
    /// Number of digits after the decimal point
    scale: usize,
    /// Sign of the number (true for positive, false for negative)
    positive: bool,
}

impl Decimal {
    pub fn try_from(input: &str) -> Option<Decimal> {
        if input.is_empty() {
            return None;
        }

        let mut chars = input.chars().peekable();

        // Handle sign
        let positive = match chars.peek() {
            Some('+') => {
                chars.next();
                true
            }
            Some('-') => {
                chars.next();
                false
            }
            _ => true,
        };

        let remaining: String = chars.collect();
        if remaining.is_empty() {
            return None;
        }

        // Split on decimal point
        let parts: Vec<&str> = remaining.split('.').collect();
        if parts.len() > 2 {
            return None; // Multiple decimal points
        }

        let integer_part = parts[0];
        let fractional_part = if parts.len() == 2 { parts[1] } else { "" };

        // Validate and collect digits
        let mut all_digits = Vec::new();

        // Process integer part (if empty, treat as "0")
        let int_part = if integer_part.is_empty() {
            "0"
        } else {
            integer_part
        };
        for c in int_part.chars() {
            if let Some(digit) = c.to_digit(10) {
                all_digits.push(digit as u8);
            } else {
                return None;
            }
        }

        // Process fractional part
        let scale = fractional_part.len();
        for c in fractional_part.chars() {
            if let Some(digit) = c.to_digit(10) {
                all_digits.push(digit as u8);
            } else {
                return None;
            }
        }

        // Handle case where we have no digits
        if all_digits.is_empty() {
            all_digits.push(0);
        }

        let result = Decimal {
            digits: all_digits,
            scale,
            positive,
        };

        Some(result.normalize())
    }

    /// Normalize the decimal by removing leading/trailing zeros and handling zero sign
    fn normalize(mut self) -> Self {
        // Handle empty digits case first
        if self.digits.is_empty() {
            self.digits.push(0);
            self.scale = 0;
            self.positive = true;
            return self;
        }

        // Remove leading zeros from integer part, but keep at least one integer digit
        // Calculate current integer digits each time since we're modifying the array
        loop {
            let current_integer_digits = if self.digits.len() <= self.scale {
                0
            } else {
                self.digits.len() - self.scale
            };

            // Stop if we would remove the last integer digit, or if no more leading zeros
            if current_integer_digits <= 1 || self.digits.is_empty() || self.digits[0] != 0 {
                break;
            }

            self.digits.remove(0);
        }

        // If we removed all integer digits but have fractional digits, add a leading zero
        let final_integer_digits = if self.digits.len() <= self.scale {
            0
        } else {
            self.digits.len() - self.scale
        };

        if final_integer_digits == 0 && self.scale > 0 {
            self.digits.insert(0, 0);
        }

        // Remove trailing zeros from fractional part
        while self.scale > 0 && !self.digits.is_empty() && self.digits.last() == Some(&0) {
            self.digits.pop();
            self.scale -= 1;
        }

        // Handle case where we might have removed all digits
        if self.digits.is_empty() {
            self.digits.push(0);
            self.scale = 0;
        }

        // Check if result is zero and normalize sign
        if self.digits.iter().all(|&d| d == 0) {
            self.positive = true;
        }

        self
    }

    /// Check if this decimal is zero
    fn is_zero(&self) -> bool {
        self.digits.iter().all(|&d| d == 0)
    }

    /// Get the number of integer digits
    fn integer_len(&self) -> usize {
        if self.digits.len() <= self.scale {
            1 // At least one zero in integer part when we have fractional digits
        } else {
            self.digits.len() - self.scale
        }
    }

    /// Compare absolute values of two decimals
    fn abs_cmp(&self, other: &Self) -> Ordering {
        let self_int_len = self.integer_len();
        let other_int_len = other.integer_len();

        // First compare integer part lengths
        match self_int_len.cmp(&other_int_len) {
            Ordering::Equal => {}
            other => return other,
        }

        // Compare integer parts digit by digit
        for i in 0..self_int_len.max(other_int_len) {
            let self_digit = if i < self_int_len && i < self.digits.len() {
                self.digits[i]
            } else {
                0
            };
            let other_digit = if i < other_int_len && i < other.digits.len() {
                other.digits[i]
            } else {
                0
            };

            match self_digit.cmp(&other_digit) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        // Compare fractional parts
        let max_frac_len = self.scale.max(other.scale);
        for i in 0..max_frac_len {
            let self_frac_idx = self.integer_len() + i;
            let other_frac_idx = other.integer_len() + i;

            let self_digit = if i < self.scale && self_frac_idx < self.digits.len() {
                self.digits[self_frac_idx]
            } else {
                0
            };
            let other_digit = if i < other.scale && other_frac_idx < other.digits.len() {
                other.digits[other_frac_idx]
            } else {
                0
            };

            match self_digit.cmp(&other_digit) {
                Ordering::Equal => continue,
                other => return other,
            }
        }

        Ordering::Equal
    }

    /// Add two decimals with same sign
    fn add_same_sign(&self, other: &Self) -> Self {
        let max_scale = self.scale.max(other.scale);

        // Convert both numbers to the same scale and add
        let mut self_digits = self.digits.clone();
        let mut other_digits = other.digits.clone();

        // Pad fractional parts with zeros to match scale
        while self_digits.len() - self.integer_len() < max_scale {
            self_digits.push(0);
        }
        while other_digits.len() - other.integer_len() < max_scale {
            other_digits.push(0);
        }

        // Pad integer parts to same length
        let self_int_len = if self_digits.len() <= max_scale {
            1
        } else {
            self_digits.len() - max_scale
        };
        let other_int_len = if other_digits.len() <= max_scale {
            1
        } else {
            other_digits.len() - max_scale
        };
        let max_int_len = self_int_len.max(other_int_len);

        while self_digits.len() - max_scale < max_int_len {
            self_digits.insert(0, 0);
        }
        while other_digits.len() - max_scale < max_int_len {
            other_digits.insert(0, 0);
        }

        let total_len = max_int_len + max_scale;
        let mut result = vec![0u8; total_len + 1]; // +1 for potential carry
        let mut carry = 0u8;

        // Add from right to left
        for i in 0..total_len {
            let pos = total_len - 1 - i;
            let self_digit = if pos < self_digits.len() {
                self_digits[pos]
            } else {
                0
            };
            let other_digit = if pos < other_digits.len() {
                other_digits[pos]
            } else {
                0
            };

            let sum = self_digit + other_digit + carry;
            result[pos + 1] = sum % 10;
            carry = sum / 10;
        }

        result[0] = carry;

        // Remove leading zeros but preserve structure
        while result.len() > max_scale + 1 && result[0] == 0 {
            result.remove(0);
        }

        Decimal {
            digits: result,
            scale: max_scale,
            positive: self.positive,
        }
        .normalize()
    }

    /// Subtract other from self (assumes |self| >= |other|)
    fn sub_abs(&self, other: &Self) -> Self {
        let max_scale = self.scale.max(other.scale);

        // Convert both numbers to the same scale
        let mut self_digits = self.digits.clone();
        let mut other_digits = other.digits.clone();

        // Pad fractional parts with zeros to match scale
        while self_digits.len() - self.integer_len() < max_scale {
            self_digits.push(0);
        }
        while other_digits.len() - other.integer_len() < max_scale {
            other_digits.push(0);
        }

        // Pad integer parts to same length
        let self_int_len = if self_digits.len() <= max_scale {
            1
        } else {
            self_digits.len() - max_scale
        };
        let other_int_len = if other_digits.len() <= max_scale {
            1
        } else {
            other_digits.len() - max_scale
        };
        let max_int_len = self_int_len.max(other_int_len);

        while self_digits.len() - max_scale < max_int_len {
            self_digits.insert(0, 0);
        }
        while other_digits.len() - max_scale < max_int_len {
            other_digits.insert(0, 0);
        }

        let total_len = max_int_len + max_scale;
        let mut result = vec![0u8; total_len];
        let mut borrow = 0u8;

        // Subtract from right to left
        for i in 0..total_len {
            let pos = total_len - 1 - i;
            let self_digit = if pos < self_digits.len() {
                self_digits[pos]
            } else {
                0
            };
            let other_digit = if pos < other_digits.len() {
                other_digits[pos]
            } else {
                0
            };

            if self_digit >= other_digit + borrow {
                result[pos] = self_digit - other_digit - borrow;
                borrow = 0;
            } else {
                result[pos] = self_digit + 10 - other_digit - borrow;
                borrow = 1;
            }
        }

        Decimal {
            digits: result,
            scale: max_scale,
            positive: true, // Sign handled by caller
        }
        .normalize()
    }
}

impl PartialEq for Decimal {
    fn eq(&self, other: &Self) -> bool {
        if self.is_zero() && other.is_zero() {
            return true;
        }

        self.positive == other.positive && self.abs_cmp(other) == Ordering::Equal
    }
}

impl Eq for Decimal {}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_zero() && other.is_zero() {
            return Ordering::Equal;
        }

        match (self.positive, other.positive) {
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (true, true) => self.abs_cmp(other),
            (false, false) => other.abs_cmp(self),
        }
    }
}

impl Add for Decimal {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (self.positive, other.positive) {
            (true, true) | (false, false) => self.add_same_sign(&other),
            (true, false) => {
                // self + (-other) = self - other
                match self.abs_cmp(&other) {
                    Ordering::Greater => {
                        let mut result = self.sub_abs(&other);
                        result.positive = true;
                        result
                    }
                    Ordering::Less => {
                        let mut result = other.sub_abs(&self);
                        result.positive = false;
                        result
                    }
                    Ordering::Equal => Decimal::try_from("0").unwrap(),
                }
            }
            (false, true) => {
                // (-self) + other = other - self
                match other.abs_cmp(&self) {
                    Ordering::Greater => {
                        let mut result = other.sub_abs(&self);
                        result.positive = true;
                        result
                    }
                    Ordering::Less => {
                        let mut result = self.sub_abs(&other);
                        result.positive = false;
                        result
                    }
                    Ordering::Equal => Decimal::try_from("0").unwrap(),
                }
            }
        }
    }
}

impl Sub for Decimal {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut neg_other = other;
        neg_other.positive = !neg_other.positive;
        self + neg_other
    }
}

impl Mul for Decimal {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        if self.is_zero() || other.is_zero() {
            return Decimal::try_from("0").unwrap();
        }

        let result_scale = self.scale + other.scale;
        let mut result = vec![0u32; self.digits.len() + other.digits.len()];

        // Standard long multiplication - multiply from right to left
        for (i, &a) in self.digits.iter().rev().enumerate() {
            for (j, &b) in other.digits.iter().rev().enumerate() {
                result[i + j] += (a as u32) * (b as u32);
            }
        }

        // Handle carries
        for i in 0..result.len() - 1 {
            result[i + 1] += result[i] / 10;
            result[i] %= 10;
        }

        // Convert back to u8 and reverse to get most significant digit first
        let mut final_result: Vec<u8> = result.into_iter().map(|x| x as u8).collect();
        final_result.reverse();

        Decimal {
            digits: final_result,
            scale: result_scale,
            positive: self.positive == other.positive,
        }
        .normalize()
    }
}
