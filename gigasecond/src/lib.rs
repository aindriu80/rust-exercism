use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = Duration::seconds(1000000000);
    println!("What time is a gigasecond later than {start}");
    start + gigasecond
}
