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

impl DurationExtension for i64 {
    fn weeks(self) -> Duration {
        Duration::weeks(self)
    }

    fn days(self) -> Duration {
        Duration::days(self)
    }

    fn hours(self) -> Duration {
        Duration::hours(self)
    }

    fn minutes(self) -> Duration {
        Duration::minutes(self)
    }

    fn seconds(self) -> Duration {
        Duration::seconds(self)
    }

    fn milliseconds(self) -> Duration {
        Duration::milliseconds(self)
    }

    fn microseconds(self) -> Duration {
        Duration::microseconds(self)
    }

    fn nanoseconds(self) -> Duration {
        Duration::nanoseconds(self)
    }
}

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
