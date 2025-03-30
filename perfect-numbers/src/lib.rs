#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(n: u64) -> Option<Classification> {
    if n == 0 {
        return None;
    }

    let mut sum = 0;

    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }

    match sum.cmp(&n) {
        std::cmp::Ordering::Greater => Some(Classification::Abundant),
        std::cmp::Ordering::Equal => Some(Classification::Perfect),
        std::cmp::Ordering::Less => Some(Classification::Deficient),
    }
}
