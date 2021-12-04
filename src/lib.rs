#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{Add, Sub};
use core::time;

#[cfg(feature = "std")]
mod std_time;

#[cfg(feature = "std")]
pub use std_time::*;

const fn usec_from_duration(duration: time::Duration) -> u64 {
    let sub_micros = (duration.subsec_nanos() / 1000) as u64;
    duration.as_secs() * 1_000_000 + sub_micros
}

fn duration_from_usec(usec: u64) -> time::Duration {
    let secs = usec / 1_000_000;
    let sub_usec = (usec % 1_000_000) as u32;
    let sub_nsec = sub_usec * 1000;
    time::Duration::new(secs, sub_nsec)
}

const fn seconds_to_micros(seconds: u64) -> u64 {
    seconds * 1_000_000
}

const fn millis_to_micros(millis: u64) -> u64 {
    millis * 1_000
}

/// An instant in monotonic time as provided by a `CLOCK_MONOTONIC` like clock
///
/// Internally, this uses microsecond (usec) sized values to track, giving 584942417355.07202148
/// years until overflow. Internally, systemd uses the same formatting for it's time values.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct MonotonicTime {
    micros: u64,
}

impl MonotonicTime {
    pub const ZERO: Self = Self { micros: 0 };

    /// Create a timestamp from microseconds
    pub const fn from_micros(micros: u64) -> Self {
        MonotonicTime { micros }
    }

    /// Return the entire timestamp converted to microseconds
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    pub const fn from_millis(millis: u64) -> Self {
        MonotonicTime {
            micros: millis_to_micros(millis),
        }
    }

    pub const fn from_seconds(seconds: u32) -> Self {
        MonotonicTime {
            micros: seconds as u64 * 1_000_000,
        }
    }
}

impl Sub<MonotonicTime> for MonotonicTime {
    type Output = Duration;

    fn sub(self, rhs: MonotonicTime) -> Self::Output {
        Duration::from_micros(self.micros - rhs.micros)
    }
}

/// Treat a duration as if it instead was a timestamp.
///
/// Sometimes, due to the lack of a way to generate a `std::time::Instant` from a numeric
/// value, other code may use a `Duration` to contain a timestamp. In that case, you should use
/// this function to obtain a `MonotonicTime` for the timestamp within the `Duration`
impl From<MonotonicTime> for time::Duration {
    fn from(s: MonotonicTime) -> time::Duration {
        duration_from_usec(s.as_micros())
    }
}

impl From<time::Duration> for MonotonicTime {
    fn from(s: time::Duration) -> Self {
        Self::from_micros(usec_from_duration(s))
    }
}

/// A timestamp from a `CLOCK_REALTIME` like clock
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct RealTime {
    micros: u64,
}

impl RealTime {
    pub const ZERO: Self = Self { micros: 0 };
}

impl Add<time::Duration> for RealTime {
    type Output = RealTime;

    fn add(self, other: time::Duration) -> RealTime {
        let mo: u64 = other.as_micros().try_into().unwrap();
        RealTime {
            micros: self.micros + mo,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct Duration {
    micros: u64,
}

impl Duration {
    pub const fn from_micros(micros: u64) -> Self {
        Duration { micros }
    }

    pub const fn from_seconds(seconds: u64) -> Self {
        Duration {
            micros: seconds_to_micros(seconds),
        }
    }

    pub const fn from_milliseconds(millis: u64) -> Self {
        Duration {
            micros: millis_to_micros(millis),
        }
    }
}
