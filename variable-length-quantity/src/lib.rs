#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    if values.is_empty() {
        return Vec::new();
    }
    if values.len() == 1 && values[0] == 0 {
        return vec![0];
    }

    let mut result = Vec::new();

    for &value in values {
        // Handle the special case where value is 0
        if value == 0 {
            result.push(0);
            continue;
        }

        // Convert the value to VLQ
        let mut bytes = Vec::new();
        let mut val = value;

        // Extract 7 bits at a time, starting from the least significant bits
        while val > 0 {
            // Take the 7 least significant bits and add to the beginning of our bytes
            // For all bytes except the last one, set the MBS to 1
            let mut byte = (val & 0x7F) as u8;
            val >>= 7;

            if !bytes.is_empty() {
                byte |= 0x80;
            }

            bytes.push(byte);

            if val == 0 {
                break;
            }
        }

        bytes.reverse();
        result.extend_from_slice(&bytes);
    }
    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::new();
    let mut i = 0;

    while i < bytes.len() {
        let mut value: u32 = 0;
        // let count = 0;
        let continuation = true;

        // Continue reading bytes until we find one without the continuation bit
        while continuation {
            if i >= bytes.len() {
                return Err(Error::IncompleteNumber);
            }

            let byte = bytes[i];
            i += 1;

            // Add the 7 bits from this byte to our value
            value = (value << 7) | (byte & 0x7F) as u32;

            // Check if we need to continue reading bytes
            if (byte & 0x80) == 0 {
                break;
            }
        }
        result.push(value);
    }
    Ok(result)
}
