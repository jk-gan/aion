use aion::*;
use chrono::Utc;

fn main() {
    println!("2 days later: {}", 2.days().later());
    println!("3 days ago: {}", 3.days().ago());
    println!("3 days ago: {}", Utc::now() - 3.days());
    println!("4 minutes later: {}", 4.minutes().later());
    println!("5 weeks later: {}", 4.weeks().later());
    println!("6 seconds ago: {}", 6.seconds().ago());
    println!("7 days from now: {}", 7.days().from_now());
    println!("8 milliseconds later: {}", 7.milliseconds().later());
}
