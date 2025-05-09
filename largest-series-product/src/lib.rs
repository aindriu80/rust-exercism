#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    // Checking if span is 0 (a special case)
    if span == 0 {
        return Ok(1); // Product of empty set is 1 by convention 
    }

    // Check if span is longer than the string
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }

    // Convert string to a vector of digits
    let digits: Result<Vec<u64>, Error> = string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|d| d as u64)
                .ok_or(Error::InvalidDigit(c))
        })
        .collect();

    let digits = digits?; // Propagate any error 

    // Find largest product using sliding window
    let max_product = (0..=string_digits.len() - span)
        .map(|i| digits[i..i + span].iter().product())
        .max()
        .unwrap_or(0);

    Ok(max_product)
}
