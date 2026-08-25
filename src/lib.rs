#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{Add, Sub};
use core::time;
use core::{fmt, result};

#[cfg(feature = "std")]
mod std_time;

const fn seconds_to_micros(seconds: u64) -> u64 {
    match checked_seconds_to_micros(seconds) {
        Some(micros) => micros,
        None => panic!("seconds do not fit in microseconds"),
    }
}

const fn millis_to_micros(millis: u64) -> u64 {
    match checked_millis_to_micros(millis) {
        Some(micros) => micros,
        None => panic!("milliseconds do not fit in microseconds"),
    }
}

const fn checked_seconds_to_micros(seconds: u64) -> Option<u64> {
    seconds.checked_mul(1_000_000)
}

const fn checked_millis_to_micros(millis: u64) -> Option<u64> {
    millis.checked_mul(1_000)
}

/// A time value is outside the range representable by this crate.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct TimeRangeError;

impl fmt::Display for TimeRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("time value is outside microtime's representable range")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TimeRangeError {}

type TimeResult<T> = result::Result<T, TimeRangeError>;

/// An instant in monotonic time as provided by a `CLOCK_MONOTONIC` like clock
///
/// Internally, this uses microsecond-sized values, giving approximately 584,554 years until
/// overflow. Systemd uses the same layout for its time values.
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct MonotonicTime {
    micros: u64,
}

/// A timestamp from a `CLOCK_REALTIME` like clock
#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct RealTime {
    micros: u64,
}

macro_rules! time_math_impl {
    ($time_type:ty) => {
        impl $time_type {
            pub const ZERO: Self = Self { micros: 0 };

            /// Create a timestamp from microseconds
            pub const fn from_micros(micros: u64) -> Self {
                Self { micros }
            }

            /// Return the entire timestamp converted to microseconds
            pub const fn as_micros(&self) -> u64 {
                self.micros
            }

            /// Create a timestamp from milliseconds.
            ///
            /// # Panics
            ///
            /// Panics if the value cannot be represented as microseconds in a `u64`.
            pub const fn from_millis(millis: u64) -> Self {
                Self {
                    micros: millis_to_micros(millis),
                }
            }

            /// Create a timestamp from milliseconds, returning `None` if it is out of range.
            pub const fn checked_from_millis(millis: u64) -> Option<Self> {
                match checked_millis_to_micros(millis) {
                    Some(micros) => Some(Self { micros }),
                    None => None,
                }
            }

            pub const fn from_seconds(seconds: u32) -> Self {
                Self {
                    micros: seconds_to_micros(seconds as u64),
                }
            }

            /// Add a duration, returning `None` if the result is out of range.
            pub const fn checked_add(self, duration: Duration) -> Option<Self> {
                match self.micros.checked_add(duration.micros) {
                    Some(micros) => Some(Self { micros }),
                    None => None,
                }
            }

            /// Return the elapsed duration, or `None` if `earlier` is later than `self`.
            pub const fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
                match self.micros.checked_sub(earlier.micros) {
                    Some(micros) => Some(Duration::from_micros(micros)),
                    None => None,
                }
            }
        }

        impl Sub for $time_type {
            type Output = Duration;

            fn sub(self, rhs: Self) -> Self::Output {
                self.checked_duration_since(rhs)
                    .expect("cannot subtract a later timestamp")
            }
        }

        impl Add<time::Duration> for $time_type {
            type Output = Self;

            fn add(self, other: time::Duration) -> Self {
                let micros =
                    micros_from_duration(other).expect("duration does not fit in microseconds");
                self.checked_add(Duration::from_micros(micros))
                    .expect("timestamp addition overflowed")
            }
        }

        impl Add<Duration> for $time_type {
            type Output = Self;

            fn add(self, other: Duration) -> Self {
                self.checked_add(other)
                    .expect("timestamp addition overflowed")
            }
        }
    };
}

time_math_impl! {MonotonicTime}
time_math_impl! {RealTime}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub struct Duration {
    micros: u64,
}

impl Duration {
    /// A duration of zero microseconds.
    pub const ZERO: Self = Self { micros: 0 };

    /// Create a duration from microseconds.
    pub const fn from_micros(micros: u64) -> Self {
        Duration { micros }
    }

    /// Create a duration from seconds.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented as microseconds in a `u64`.
    pub const fn from_seconds(seconds: u64) -> Self {
        Duration {
            micros: seconds_to_micros(seconds),
        }
    }

    /// Create a duration from seconds, returning `None` if it is out of range.
    pub const fn checked_from_seconds(seconds: u64) -> Option<Self> {
        match checked_seconds_to_micros(seconds) {
            Some(micros) => Some(Self { micros }),
            None => None,
        }
    }

    /// Create a duration from milliseconds.
    ///
    /// # Panics
    ///
    /// Panics if the value cannot be represented as microseconds in a `u64`.
    pub const fn from_milliseconds(millis: u64) -> Self {
        Duration {
            micros: millis_to_micros(millis),
        }
    }

    /// Create a duration from milliseconds, returning `None` if it is out of range.
    pub const fn checked_from_milliseconds(millis: u64) -> Option<Self> {
        match checked_millis_to_micros(millis) {
            Some(micros) => Some(Self { micros }),
            None => None,
        }
    }

    /// Return the entire duration in microseconds.
    pub const fn as_micros(&self) -> u64 {
        self.micros
    }

    /// Add two durations, returning `None` if the result is out of range.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.micros.checked_add(rhs.micros) {
            Some(micros) => Some(Self { micros }),
            None => None,
        }
    }

    /// Subtract two durations, returning `None` if `rhs` is greater than `self`.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        match self.micros.checked_sub(rhs.micros) {
            Some(micros) => Some(Self { micros }),
            None => None,
        }
    }
}

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        self.checked_add(rhs).expect("duration addition overflowed")
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self.checked_sub(rhs)
            .expect("duration subtraction underflowed")
    }
}

const fn duration_from_micros(micros: u64) -> time::Duration {
    let secs = micros / 1_000_000;
    let sub_micros = (micros % 1_000_000) as u32;
    let sub_nsec = sub_micros * 1000;
    time::Duration::new(secs, sub_nsec)
}

fn micros_from_duration(duration: time::Duration) -> TimeResult<u64> {
    duration.as_micros().try_into().map_err(|_| TimeRangeError)
}

impl MonotonicTime {
    /// Treat a `MonotonicTime` as if it were a `core::time::Duration`
    ///
    /// Sometimes, due to the lack of a way to generate a `std::time::Instant` from a numeric
    /// value, other code may use a `Duration` to contain a timestamp. This creates such a
    /// duration.
    pub fn to_core_duration(self) -> core::time::Duration {
        duration_from_micros(self.micros)
    }

    /// Treat a duration as if it instead was a timestamp.
    ///
    /// Sometimes, due to the lack of a way to generate a `std::time::Instant` from a numeric
    /// value, other code may use a `Duration` to contain a timestamp.  In that case, you should
    /// use this function to obtain a `MonotonicTime` for the timestamp within the `Duration`
    ///
    /// # Panics
    ///
    /// Panics if the duration cannot be represented in microseconds in a `u64`.
    pub fn from_core_duration(ctd: core::time::Duration) -> Self {
        Self::try_from(ctd).expect("duration does not fit in microseconds")
    }
}

impl TryFrom<time::Duration> for MonotonicTime {
    type Error = TimeRangeError;

    fn try_from(duration: time::Duration) -> TimeResult<Self> {
        Ok(Self::from_micros(micros_from_duration(duration)?))
    }
}
