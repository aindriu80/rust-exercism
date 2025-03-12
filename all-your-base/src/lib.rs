#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    //  Validate the bases
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    //  Validate the digits
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
    }

    //  Convert from input base to decimal (base 10)
    let mut decimal_value: u64 = 0;
    for &digit in number {
        decimal_value = decimal_value * (from_base as u64) + (digit as u64);
    }

    //  Convert from decimal to output base
    if decimal_value == 0 {
        return Ok(vec![0]); // Always return [0] for zero, regardless of input
    }

    let mut result_digits = Vec::new();
    let mut temp_value = decimal_value;
    while temp_value > 0 {
        let remainder = (temp_value % (to_base as u64)) as u32;
        result_digits.insert(0, remainder);
        temp_value /= to_base as u64;
    }

    //  Return the result
    Ok(result_digits)
}
