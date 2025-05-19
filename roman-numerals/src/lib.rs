use std::fmt::{Display, Formatter, Result};

pub struct Roman {
    value: String,
}

impl Display for Roman {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.value)
    }
}

impl From<u32> for Roman {
    fn from(num: u32) -> Self {
        if num == 0 {
            return Roman {
                value: String::new(),
            };
        }
        let symbols = [
            (1000, "M"),
            (900, "CM"),
            (500, "D"),
            (400, "CD"),
            (100, "C"),
            (90, "XC"),
            (50, "L"),
            (40, "XL"),
            (10, "X"),
            (9, "IX"),
            (5, "V"),
            (4, "IV"),
            (1, "I"),
        ];

        let mut result = String::new();
        let mut remaining = num;

        // iterate through each symbol and value pair
        for &(value, symbol) in symbols.iter() {
            // While the remaining number is at least the current value
            while remaining >= value {
                result.push_str(symbol); // Append the symbol 
                remaining -= value; // Subtract the value
            }
        }
        Roman { value: result }
    }
}
