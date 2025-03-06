pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    for item in string.chars() {
        match item {
            '[' | '{' | '(' => stack.push(item),
            ')' => {
                if stack.pop() != Some('(') {
                    return false;
                }
            }
            '}' => {
                if stack.pop() != Some('{') {
                    return false;
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    // return true;
    stack.is_empty()
}
