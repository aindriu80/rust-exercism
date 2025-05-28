pub fn actions(n: u8) -> Vec<&'static str> {
    let messages = ["wink", "double blink", "close your eyes", "jump"];
    let mut result = Vec::new();

    for i in 0..4 {
        if n & (1 << i) != 0 {
            result.push(messages[i]);
        }
    }

    if n & (1 << 4) != 0 {
        result.reverse();
    }
    result
}
