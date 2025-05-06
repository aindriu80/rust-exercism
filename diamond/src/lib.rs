pub fn get_diamond(c: char) -> Vec<String> {
    if !c.is_alphabetic() {
        return vec![];
    }

    if c == 'A' {
        return vec!["A".to_string()];
    }

    let input_char = c.to_ascii_uppercase();

    let char_distance = (input_char as u8 - 'A' as u8) as usize;
    let diamond_size = char_distance * 2 + 1;

    let mut result = Vec::with_capacity(diamond_size);

    for i in 0..=char_distance {
        let current_char = (('A' as u8) + i as u8) as char;

        if current_char == 'A' {
            let padding = " ".repeat(char_distance);
            result.push(format!("{}A{}", padding, padding));
        } else {
            let outer_padding = " ".repeat(char_distance - i);
            let inner_spacing = " ".repeat(i * 2 - 1);

            result.push(format!(
                "{}{}{}{}{}",
                outer_padding, current_char, inner_spacing, current_char, outer_padding
            ));
        }
    }

    for i in (0..char_distance).rev() {
        result.push(result[i].clone());
    }

    result
}
