pub fn recite(start_bottles: u32, take_down: u32) -> String {
    fn number_to_word(n: u32) -> &'static str {
        match n {
            10 => "Ten",
            9 => "Nine",
            8 => "Eight",
            7 => "Seven",
            6 => "Six",
            5 => "Five",
            4 => "Four",
            3 => "Three",
            2 => "Two",
            1 => "One",
            _ => "No",
        }
    }

    let mut verses = Vec::new();

    for i in (start_bottles.saturating_sub(take_down - 1)..=start_bottles).rev() {
        let current = number_to_word(i);
        let next = number_to_word(i.saturating_sub(1));

        let plural = if i == 1 { "bottle" } else { "bottles" };
        let next_plural = if i.saturating_sub(1) == 1 {
            "bottle"
        } else {
            "bottles"
        };

        let verse = format!(
            "{0} green {1} hanging on the wall,\n\
             {0} green {1} hanging on the wall,\n\
             And if one green bottle should accidentally fall,\n\
             There'll be {2} green {3} hanging on the wall.",
            current,
            plural,
            next.to_lowercase(),
            next_plural
        );

        verses.push(verse);
    }

    verses.join("\n\n")
}
