use std::fmt;
use std::ops::Add;
use std::ops::Div;
use std::ops::Sub;

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Duration {
    pub millis: u64,
}

impl Sub for Duration {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration {
            millis: self.millis - rhs.millis,
        }
    }
}

impl Add for Duration {
    type Output = Duration;

    fn add(self, rhs: Self) -> Self::Output {
        Duration {
            millis: self.millis + rhs.millis,
        }
    }
}

impl Div<u64> for Duration {
    type Output = Duration;

    fn div(self, rhs: u64) -> Self::Output {
        Duration {
            millis: self.millis / rhs,
        }
    }
}

impl Div<Duration> for Duration {
    type Output = f64;

    fn div(self, rhs: Duration) -> Self::Output {
        self.millis as f64 / rhs.millis as f64
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{:03}", self.millis / 1000, self.millis % 1000)
    }
}