pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None; // Invalid input
    }

    let mut current = n; // mutable variable to track the current number
    let mut steps = 0; // Count the number of steps

    while current != 1 {
        // Check for potential overflow before multiplying
        if current % 2 != 0 && current > (u64::MAX - 1) / 3 {
            return None; // Overflow would occur in 3n + 1
        }

        if current % 2 == 0 {
            current = current / 2; // Even: divide by 2
        } else {
            current = (current * 3) + 1; // Odd: multiply by 3 and add 1
        }
        steps += 1;

        // Optional: Guard against infinite loops (e.g., for testing)
        if steps > 1_000_000 {
            return None; // Arbitary limit to avoid hangs
        }
    }
    Some(steps) // Return the number of steps
}
