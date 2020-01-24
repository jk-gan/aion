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
        impl DurationExtension for i32 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
                }
            )*
        }
        impl DurationExtension for i16 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
                }
            )*
        }
        impl DurationExtension for i8 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
                }
            )*
        }
        impl DurationExtension for u32 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
                }
            )*
        }
        impl DurationExtension for u16 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
                }
            )*
        }
        impl DurationExtension for u8 {
            $(
                fn $type(self) -> Duration {
                    Duration::$type(self.into())
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
    fn i32_duration() {
        assert_eq!(2i32.weeks(), Duration::weeks(2));
        assert_eq!(2i32.days(), Duration::days(2));
        assert_eq!(2i32.hours(), Duration::hours(2));
        assert_eq!(2i32.minutes(), Duration::minutes(2));
        assert_eq!(2i32.seconds(), Duration::seconds(2));
        assert_eq!(2i32.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2i32.microseconds(), Duration::microseconds(2));
        assert_eq!(2i32.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn i16_duration() {
        assert_eq!(2i16.weeks(), Duration::weeks(2));
        assert_eq!(2i16.days(), Duration::days(2));
        assert_eq!(2i16.hours(), Duration::hours(2));
        assert_eq!(2i16.minutes(), Duration::minutes(2));
        assert_eq!(2i16.seconds(), Duration::seconds(2));
        assert_eq!(2i16.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2i16.microseconds(), Duration::microseconds(2));
        assert_eq!(2i16.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn i8_duration() {
        assert_eq!(2i8.weeks(), Duration::weeks(2));
        assert_eq!(2i8.days(), Duration::days(2));
        assert_eq!(2i8.hours(), Duration::hours(2));
        assert_eq!(2i8.minutes(), Duration::minutes(2));
        assert_eq!(2i8.seconds(), Duration::seconds(2));
        assert_eq!(2i8.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2i8.microseconds(), Duration::microseconds(2));
        assert_eq!(2i8.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn u32_duration() {
        assert_eq!(2u32.weeks(), Duration::weeks(2));
        assert_eq!(2u32.days(), Duration::days(2));
        assert_eq!(2u32.hours(), Duration::hours(2));
        assert_eq!(2u32.minutes(), Duration::minutes(2));
        assert_eq!(2u32.seconds(), Duration::seconds(2));
        assert_eq!(2u32.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2u32.microseconds(), Duration::microseconds(2));
        assert_eq!(2u32.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn u16_duration() {
        assert_eq!(2u16.weeks(), Duration::weeks(2));
        assert_eq!(2u16.days(), Duration::days(2));
        assert_eq!(2u16.hours(), Duration::hours(2));
        assert_eq!(2u16.minutes(), Duration::minutes(2));
        assert_eq!(2u16.seconds(), Duration::seconds(2));
        assert_eq!(2u16.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2u16.microseconds(), Duration::microseconds(2));
        assert_eq!(2u16.nanoseconds(), Duration::nanoseconds(2));
    }

    #[test]
    fn u8_duration() {
        assert_eq!(2u8.weeks(), Duration::weeks(2));
        assert_eq!(2u8.days(), Duration::days(2));
        assert_eq!(2u8.hours(), Duration::hours(2));
        assert_eq!(2u8.minutes(), Duration::minutes(2));
        assert_eq!(2u8.seconds(), Duration::seconds(2));
        assert_eq!(2u8.milliseconds(), Duration::milliseconds(2));
        assert_eq!(2u8.microseconds(), Duration::microseconds(2));
        assert_eq!(2u8.nanoseconds(), Duration::nanoseconds(2));
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
