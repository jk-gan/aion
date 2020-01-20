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
}
