use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (points, letters) in h {
        for &letter in letters {
            result.insert(letter.to_ascii_lowercase(), *points);
        }
    }

    result
}
