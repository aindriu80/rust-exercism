pub fn find(array: &[i32], target: i32) -> Option<usize> {
    if array.is_empty() {
        return None; // Handle empty array case
    }
    // Early exit if target is out of bounds
    if target < array[0] || target > array[array.len() - 1] {
        return None;
    }

    let mut left = 0;
    let mut right = array.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2; // Safe midpoint calculation
        let current = array[mid];

        if current == target {
            return Some(mid);
        } else if current < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None // Implicit return when target isn’t found
}
