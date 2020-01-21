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
    fn before(self, other: DateTime<Utc>) -> DateTime<Utc>;
    fn after(self, other: DateTime<Utc>) -> DateTime<Utc>;

    fn ago(self) -> DateTime<Utc>
    where
        Self: DateTimeExtension + Sized,
    {
        self.before(Utc::now())
    }

    fn from_now(self) -> DateTime<Utc>
    where
        Self: DateTimeExtension + Sized,
    {
        self.after(Utc::now())
    }
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
    fn before(self, other: DateTime<Utc>) -> DateTime<Utc> {
        other - self
    }

    fn after(self, other: DateTime<Utc>) -> DateTime<Utc> {
        other + self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn duration() {
        assert_eq!(2.weeks(), Duration::weeks(2));
        assert_eq!(2.days(), Duration::days(2));
        assert_eq!(2.hours(), Duration::hours(2));
        assert_eq!(2.minutes(), Duration::minutes(2));
        assert_eq!(2.seconds(), Duration::seconds(2));
        assert_eq!(2.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2.microseconds(), Duration::microseconds(2));
        assert_eq!(2.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn days_before() {
        let now = Utc::now();
        assert_eq!(2.days().before(now), now - Duration::days(2));
    }

    #[test]
    fn days_after() {
        let now = Utc::now();
        assert_eq!(2.days().after(now), now + Duration::days(2));
    }
}
