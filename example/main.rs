use aion::*;
use chrono::{TimeZone, Utc};

fn main() {
    // Easily represent a chrono::Duration
    let two_days = 2.days();
    println!("2 days: {:?}", two_days);
    let attention_span = 1.seconds();
    println!("Attention span: {:?}", attention_span);

    // Add or subtract durations from the current time (UTC)
    let two_hours_from_now = 2.hours().from_now();
    println!("2 hours from now: {}", two_hours_from_now);
    let last_week = 7.days().ago(); // or 1.weeks().ago()
    println!("1 week ago: {}", last_week);

    // More complex DateTimes can be represented using before() and after() methods
    let christmas = Utc.ymd(2020, 12, 25).and_hms(0, 0, 0);
    let two_weeks_before_christmas = 2.weeks().before(christmas);
    println!("2 weeks before christmas: {}", two_weeks_before_christmas);
    let boxing_day = 1.days().after(christmas);
    println!("Boxing day: {}", boxing_day);
}
