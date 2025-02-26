pub fn is_leap_year(year: u64) -> bool {
    let is_divisible: u64 = year;

    if is_divisible % 100 == 0 {
        if is_divisible % 400 == 0 {
            return true;
        }
    } else if is_divisible % 4 == 0 {
        return true;
    }
    return false;
}
