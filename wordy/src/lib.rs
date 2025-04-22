pub fn answer(command: &str) -> Option<i32> {
    if command.is_empty() || !command.starts_with("What is") {
        return None;
    }
    // Remove "What is" and the optional trailing "?"
    let expr = command
        .trim_start_matches("What is")
        .trim()
        .trim_end_matches('?')
        .trim();
    if expr.is_empty() {
        return None;
    }

    // Tokenize the input
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.is_empty() {
        return None;
    }

    // First token must be a number
    let mut result = match tokens[0].parse::<i32>() {
        Ok(num) => num,
        Err(_) => return None, // Reject prefix notation
    };

    // Process tokens in pairs (operator, operand)
    let mut i = 1;
    while i < tokens.len() {
        // Ensure we have enough tokens
        if i + 1 >= tokens.len() {
            return None; // Malformed expression
        }

        // Process operator
        let op = tokens[i];
        i += 1;

        // Handle "by" for multiplication and division
        if (op == "multiplied" || op == "divided") && i < tokens.len() && tokens[i] == "by" {
            i += 1;
        }

        // Handle exponentiation (if enabled) - commented out since it's not in your current requirements
        /*
        #[cfg(feature = "exponentials")]
        if op == "raised" && i + 2 < tokens.len() && tokens[i] == "to" && tokens[i+1] == "the" && tokens[i+2].starts_with("power") {
            i += 3; // Skip "to the power"
        }
        */

        // Ensure we have a number after the operator
        if i >= tokens.len() {
            return None; // Malformed expression
        }

        // Parse the operand
        let operand = match tokens[i].parse::<i32>() {
            Ok(num) => num,
            Err(_) => return None, // Reject if not a number
        };
        i += 1;

        // Perform the operation
        result = match op {
            "plus" => result + operand,
            "minus" => result - operand,
            "multiplied" => result * operand,
            "divided" => {
                if operand == 0 {
                    return None; // Division by zero
                }
                result / operand
            }
            _ => return None, // Unknown operator
        };
    }

    // If we've consumed all tokens, return the result
    if i == tokens.len() {
        Some(result)
    } else {
        None // There are extra tokens we didn't process
    }
}
