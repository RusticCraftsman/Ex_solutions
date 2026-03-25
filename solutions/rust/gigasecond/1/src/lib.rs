use time::{Duration, PrimitiveDateTime as DateTime};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
    let gigasecond = 1000000000;
    start.saturating_add(Duration::seconds(gigasecond))
}
