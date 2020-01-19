use chrono::{DateTime, Duration, Utc};

pub trait Day {
    fn days(self) -> Duration;
}

pub trait Later {
    fn later(self) -> DateTime<Utc>;
}

pub trait Ago {
    fn ago(self) -> DateTime<Utc>;
}

impl Day for i64 {
    fn days(self) -> Duration {
        Duration::days(self)
    }
}

impl Later for Duration {
    fn later(self) -> DateTime<Utc> {
        Utc::now() + self
    }
}

impl Ago for Duration {
    fn ago(self) -> DateTime<Utc> {
        Utc::now() - self
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
        assert_eq!(2.days(), Duration::days(2));
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
