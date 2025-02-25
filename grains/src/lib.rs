pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Invalid square number");
    }
    2u64.pow(s - 1)
}

pub fn total() -> u64 {
    let mut sum = 0;
    for i in 1..65 {
        sum += square(i);
    }
    sum
}
