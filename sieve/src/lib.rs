pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    if upper_bound <= 1 {
        return Vec::new();
    }
    let mut is_prime = vec![true; upper_bound as usize + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..upper_bound {
        if is_prime[i as usize] {
            let mut j = i * i;
            while j <= upper_bound {
                is_prime[j as usize] = false;
                j += i;
            }
        }
    }

    let mut result = Vec::new();
    for i in 2..=upper_bound {
        if is_prime[i as usize] {
            result.push(i);
        }
    }

    result
}
