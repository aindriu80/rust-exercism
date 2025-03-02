pub fn raindrops(n: u32) -> String {
    let mut result: Vec<String> = vec![];

    if n % 3 == 0 {
        result.push("Pling".to_string());
    }
    if n % 5 == 0 {
        result.push("Plang".to_string());
    }
    if n % 7 == 0 {
        result.push("Plong".to_string());
    }
    if result.len() == 0 {
        result.push(n.to_string());
    }

    result.join("")
}
