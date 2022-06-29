use chrono::{Datelike, Timelike, Weekday};

fn now() -> (Weekday, u32) {
    let time = chrono::Utc::now();
    let weekday = time.date().weekday();
    let hour = time.hour();
    return (weekday, hour)
}

fn main() {}
