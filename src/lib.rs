use chrono::{DateTime, Duration, Utc};

pub trait DurationExtension {
    fn weeks(self) -> Duration;
    fn days(self) -> Duration;
    fn hours(self) -> Duration;
    fn minutes(self) -> Duration;
    fn seconds(self) -> Duration;
    fn milliseconds(self) -> Duration;
    fn microseconds(self) -> Duration;
    fn nanoseconds(self) -> Duration;
}

pub trait DateTimeExtension {
    fn ago(self) -> DateTime<Utc>;
    fn later(self) -> DateTime<Utc>;
    fn from_now(self) -> DateTime<Utc>;
}

macro_rules! duration_extension {
    ($($type:ident), +) => {
        impl DurationExtension for i64 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self)
                }
            )*
        }
    };
}
duration_extension!(
    weeks,
    days,
    hours,
    minutes,
    seconds,
    milliseconds,
    microseconds,
    nanoseconds
);

impl DateTimeExtension for Duration {
    fn ago(self) -> DateTime<Utc> {
        Utc::now() - self
    }

    fn later(self) -> DateTime<Utc> {
        Utc::now() + self
    }

    fn from_now(self) -> DateTime<Utc> {
        Utc::now() + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn duration() {
        assert_eq!(2.weeks(), Duration::weeks(2));
        assert_eq!(2.days(), Duration::days(2));
        assert_eq!(2.hours(), Duration::hours(2));
        assert_eq!(2.minutes(), Duration::minutes(2));
        assert_eq!(2.seconds(), Duration::seconds(2));
    }

    #[test]
    fn days_later() {
        assert_eq!(2.days().later(), Utc::now() + Duration::days(2));
    }

    #[test]
    fn days_ago() {
        assert_eq!(2.days().ago(), Utc::now() - Duration::days(2));
    }
}
