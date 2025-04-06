pub fn encode(num: u64) -> String {
    if num == 0 {
        return "zero".to_string();
    }

    // Names for the different scales
    let scales = [
        "",
        "thousand",
        "million",
        "billion",
        "trillion",
        "quadrillion",
        "quintillion",
    ];

    // Break the number into chunks of 3 digits
    let mut chunks = Vec::new();
    let mut remaining = num;

    while remaining > 0 {
        chunks.push(remaining % 1000);
        remaining /= 1000;
    }

    let mut result = String::new();

    // Process each chunk with its scale
    for i in (0..chunks.len()).rev() {
        if chunks[i] != 0 {
            let chunk_words = spell_chunk(chunks[i]);

            if !result.is_empty() {
                result.push(' ');
            }

            result.push_str(&chunk_words);

            if i > 0 {
                result.push_str(" ");
                result.push_str(scales[i]);
            }
        }
    }

    result
}

// Handle numbers from 0-999 non-recursively
fn spell_chunk(num: u64) -> String {
    let units = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    let mut result = String::new();

    // Handle hundreds
    if num >= 100 {
        result.push_str(units[(num / 100) as usize]);
        result.push_str(" hundred");

        if num % 100 != 0 {
            result.push_str(" ");
            // Add "and" here if you want British English style
            // result.push_str("and ");
        }
    }

    // Handle tens and units
    let remainder = num % 100;
    if remainder > 0 {
        if remainder < 20 {
            result.push_str(units[remainder as usize]);
        } else {
            result.push_str(tens[(remainder / 10) as usize]);

            if remainder % 10 != 0 {
                result.push_str("-");
                result.push_str(units[(remainder % 10) as usize]);
            }
        }
    }

    result
}
