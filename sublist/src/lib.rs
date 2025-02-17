#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(first_list, second_list) {
        return Comparison::Sublist;
    }

    if is_sublist(second_list, first_list) {
        return Comparison::Superlist;
    }

    Comparison::Unequal
}

fn is_sublist(sub: &[i32], full: &[i32]) -> bool {
    if sub.is_empty() {
        return true;
    }

    full.windows(sub.len()).any(|window| window == sub)
}
