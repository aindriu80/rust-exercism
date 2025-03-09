use std::collections::HashMap;

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let students = [
        "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
        "Kincaid", "Larry",
    ];

    let plant_map: HashMap<char, &str> = vec![
        ('G', "grass"),
        ('C', "clover"),
        ('R', "radishes"),
        ('V', "violets"),
    ]
    .into_iter()
    .collect();

    let student_index = students.iter().position(|&s| s == student).unwrap_or(0);
    let start = student_index * 2;
    let end = start + 2;

    let rows: Vec<&str> = diagram.split('\n').collect();
    if rows.len() != 2 {
        return vec![];
    }

    let mut result = Vec::new();
    for row in &rows {
        result.extend(row.chars().skip(start).take(2).map(|c| plant_map[&c]));
    }

    result
}
