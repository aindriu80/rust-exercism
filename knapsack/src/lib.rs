#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if items.is_empty() || max_weight == 0 {
        return 0;
    }
    let max_weight = max_weight as usize;
    let n = items.len();

    let mut dp = vec![vec![0u32; max_weight + 1]; n + 1];

    for i in 1..=n {
        let item = &items[i - 1];
        let item_weight = item.weight as usize;

        for w in 0..=max_weight {
            // Don't take the current item
            dp[i][w] = dp[i - 1][w];

            // Take the current item if it fits
            if item_weight <= w {
                let value_with_item = dp[i - 1][w - item_weight] + item.value;
                dp[i][w] = dp[i][w].max(value_with_item);
            }
        }
    }
    dp[n][max_weight]
}
