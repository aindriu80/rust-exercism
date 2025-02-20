pub fn is_armstrong_number(num: i32) -> bool {
    let num_str = num.to_string();
    let num_len = num_str.len();

    let mut sum = 0;
    for c in num_str.chars() {
        let digit = c.to_digit(10).unwrap() as u64;
        sum += digit.pow(num_len as u32);
    }

    if sum > i64::MAX as u64 {
        panic!("The result is too big to fit into a `u64`.");
    } else {
        sum == num.try_into().unwrap()
    }
}
