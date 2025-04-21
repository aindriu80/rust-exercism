pub fn answer(command: &str) -> Option<i32> {
    if command.is_empty() {
        return None;
    }
    let numbers: Vec<i32> = command
        .split_whitespace()
        .filter_map(|word| word.parse::<i32>().ok())
        .collect();

    // Make sure we have at least two numbers
    if numbers.len() < 2 {
        return None;
    }

    let first_number = numbers[0];
    let second_number = numbers[1];

    // Determine the operation
    if command.contains("plus") {
        Some(first_number + second_number)
    } else if command.contains("minus") {
        Some(first_number - second_number)
    } else if command.contains("multiplied") {
        Some(first_number * second_number)
    } else if command.contains("divided") {
        // Check for division by zero
        if second_number == 0 {
            return None;
        }
        Some(first_number / second_number)
    } else {
        None
    }
}
