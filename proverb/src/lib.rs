pub fn build_proverb(list: &[&str]) -> String {
    // Handle empty list case
    if list.is_empty() {
        return String::new();
    }

    // Handle single item case and build the full proverb for multiple items
    let mut lines: Vec<String> = Vec::new();

    // Generate the "For want of" lines, but only if we have 2 or more items
    for i in 0..list.len().saturating_sub(1) {
        let current = list[i];
        let next = list[i + 1];
        lines.push(format!("For want of a {} the {} was lost.", current, next));
    }

    // Add the final line referencing the first item
    if !list.is_empty() {
        lines.push(format!("And all for the want of a {}.", list[0]));
    }

    // Join all lines with newlines
    lines.join("\n")
}
