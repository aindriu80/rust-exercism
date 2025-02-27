pub fn nth(n: u32) -> u32 {
    // If n is 0, return the first prime immediately
    if n == 0 {
        return 2;
    }

    let mut count = 0; // Number of primes found so far
    let mut candidate = 3; // Start checking from 3 (since 2 is handled)

    // Loop until we find the nth prime
    while count < n {
        if is_prime(candidate) {
            count += 1; // Increment count when we find a prime
        }
        // Move to the next odd number (since even numbers > 2 aren't prime)
        candidate += 2;
    }

    candidate - 2 // Subtract 2 because we overshoot by one increment
}

// Helper function to check if a number is prime
fn is_prime(num: u32) -> bool {
    if num < 2 {
        return false;
    }
    // Check divisibility up to the square root of num
    for i in 2..=(num as f64).sqrt() as u32 {
        if num % i == 0 {
            return false;
        }
    }
    true
}
