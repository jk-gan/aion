## Aion [![](http://meritbadge.herokuapp.com/aion)](https://crates.io/crates/aion)
Aion is a utility crate, inspired by rails, that allows you to write `Duration` and `DateTime` in a friendly way:
```rust
// Easily represent a chrono::Duration
let two_days = 2.days();
let attention_span = 1.seconds();

// Add or subtract durations from the current time (UTC)
let now = Utc::now();
let three_hours_from_now = now + 3.hours();
let two_hours_from_now = 2.hours().from_now();
let last_week = 7.days().ago(); // or 1.weeks().ago()

// More complex DateTimes can be represented using before() and after() methods
let christmas = Utc.ymd(2020, 12, 25).and_hms(0, 0, 0);
let two_weeks_before_christmas = 2.weeks().before(christmas);
let boxing_day = 1.days().after(christmas);
```

## Installation
Add this to your `Cargo.toml` file:
```toml
[dependencies]
aion = "0.2"
```

## Example
The example is located in `example` folder. You can run it by using
```bash
cargo run --example example
```

## Limitations
Currently this crate only will return `DateTime<Utc>`.

## Acknowledgement
This crate is using [chrono](https://github.com/chronotope/chrono). Thanks for this awesome crate.
