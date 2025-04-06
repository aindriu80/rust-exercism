pub fn encode(num: u64) -> String {
    if num == 0 {
        return "zero".to_string();
    }

    spell_number(num)
}

fn spell_number(num: u64) -> String {
    let units = [
        "",
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
        "ten",
        "eleven",
        "twelve",
        "thirteen",
        "fourteen",
        "fifteen",
        "sixteen",
        "seventeen",
        "eighteen",
        "nineteen",
    ];
    let tens = [
        "", "", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if num < 20 {
        return units[num as usize].to_string();
    } else if num < 100 {
        let ten = tens[(num / 10) as usize];
        let unit = units[(num % 10) as usize];

        if unit.is_empty() {
            return ten.to_string();
        } else {
            return format!("{}-{}", ten, unit);
        }
    } else if num < 1000 {
        let hundred = units[(num / 100) as usize];
        let remainder = num % 100;

        if remainder == 0 {
            return format!("{} hundred", hundred);
        } else {
            return format!("{} hundred {}", hundred, spell_number(remainder));
        }
    } else if num < 1_000_000 {
        let thousand = spell_number(num / 1000);
        let remainder = num % 1000;

        if remainder == 0 {
            return format!("{} thousand", thousand);
        } else {
            return format!("{} thousand {}", thousand, spell_number(remainder));
        }
    } else if num < 1_000_000_000 {
        let million = spell_number(num / 1_000_000);
        let remainder = num % 1_000_000;

        if remainder == 0 {
            return format!("{} million", million);
        } else {
            return format!("{} million {}", million, spell_number(remainder));
        }
    } else if num < 1_000_000_000_000 {
        let billion = spell_number(num / 1_000_000_000);
        let remainder = num % 1_000_000_000;

        if remainder == 0 {
            return format!("{} billion", billion);
        } else {
            return format!("{} billion {}", billion, spell_number(remainder));
        }
    } else {
        let quintillion = spell_number(num / 1_000_000_000_000_000_000);
        let remainder = num % 1_000_000_000_000_000_000;

        if remainder == 0 {
            return format!("{} quintillion", quintillion);
        } else {
            return format!("{}quintillion {}", quintillion, spell_number(remainder));
        }
    }
}
