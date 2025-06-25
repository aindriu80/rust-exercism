use std::collections::HashMap;

pub fn lowest_price(books: &[u32]) -> u32 {
    if books.is_empty() {
        return 0;
    }

    // Count how many copies of each book we have
    let mut book_counts = HashMap::new();
    for &book in books {
        *book_counts.entry(book).or_insert(0) += 1;
    }

    // Convert to a vector of counts for easier processing
    let mut counts: Vec<u32> = book_counts.values().cloned().collect();
    counts.sort_by(|a, b| b.cmp(a)); // Sort in descending order

    calculate_min_price(&counts)
}

fn calculate_min_price(counts: &[u32]) -> u32 {
    if counts.is_empty() || counts.iter().all(|&count| count == 0) {
        return 0;
    }
    let mut min_price = u32::MAX;

    // Try all possible group sizes (1 to 5, but we'll focus on 2-5 for discount)
    for group_size in 1..=5 {
        if let Some(price) = try_grouping(counts, group_size) {
            min_price = min_price.min(price);
        }
    }
    min_price
}

fn try_grouping(counts: &[u32], group_size: usize) -> Option<u32> {
    // Check if we can form a group of this size
    let available_books = counts.iter().filter(|&&count| count > 0).count();
    if available_books < group_size {
        return None;
    }

    // Create a group by taking one back from he top `group_size` stacks
    let mut new_counts = counts.to_vec();
    let mut books_taken = 0;

    for i in 0..new_counts.len() {
        if books_taken < group_size && new_counts[i] > 0 {
            new_counts[i] -= 1;
            books_taken += 1;
        }
    }

    if books_taken < group_size {
        return None;
    }

    // Calculate price for this group
    let group_price = calculate_group_price(group_size);

    // Recursively calculate the price for remaining books
    let remaining_price = calculate_min_price(&new_counts);

    Some(group_price + remaining_price)
}

fn calculate_group_price(group_size: usize) -> u32 {
    let base_price = 800; // Price per book in cents 
    let total_base = (group_size as u32) * base_price;

    let discount_percent = match group_size {
        1 => 0,
        2 => 5,
        3 => 10,
        4 => 20,
        5 => 25,
        _ => 0,
    };

    // Apply discount
    let discount = (total_base * discount_percent) / 100;
    total_base - discount
}
