## Aion
Aion is a utility crate, inspired by rails, that allows you to write `Duration` and `DateTime` in a friendly way:
```rust
let now = Utc::now();
let two_days_later_1 = now + 2.days(); 
let two_days_later_2 = 2.days().from_now();
```

## Installation
Add this to your `Cargo.toml` file:
```toml
[dependencies]
aion = "0.1"
```

## Example
```rust
use aion::*;
use chrono::Utc;

fn main() {
    println!("2 days from now: {}", 2.days().from_now());
    println!("3 days ago: {}", 3.days().ago());
    println!("3 days ago: {}", Utc::now() - 3.days());
    println!("4 minutes from now: {}", 4.minutes().from_now());
    println!("5 weeks from now: {}", 5.weeks().from_now());
    println!("6 seconds ago: {}", 6.seconds().ago());
    println!("7 days from now: {}", 7.days().from_now());
    println!("8 milliseconds from now: {}", 8.milliseconds().from_now());
    println!(
        "5 days ago + 9 microseconds: {}",
        9.microseconds().after(5.days().ago())
    );
    println!(
        "2 weeks from now - 10 nanoseconds: {}",
        10.nanoseconds().before(2.weeks().from_now())
    );
}
```

## Limitations
Currently this crate only will return `DateTime<Utc>`.

## Acknowledgement
This crate is using [chrono](https://github.com/chronotope/chrono). Thanks for this awesome crate.
