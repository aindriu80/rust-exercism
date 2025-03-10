pub fn egg_count(display_value: u32) -> usize {
    if display_value == 0 {
        return 0;
    }
    let binary = format!("{:b}", display_value);
    let mut ones = 0;
    for c in binary.chars() {
        if c == '1' {
            ones += 1;
        }
    }
    return ones;
}
