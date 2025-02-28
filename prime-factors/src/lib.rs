pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut num = n;
    let mut i = 2;

    while i * i <= num {
        if num % i == 0 {
            factors.push(i);
            num /= i;
        } else {
            i += 1;
        }
    }

    if num > 1 {
        factors.push(num);
    }

    factors
}
