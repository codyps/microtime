#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::{Add, Sub};
use core::time;

#[cfg(feature = "std")]
mod std_time;

#[cfg(feature = "std")]
pub use std_time::*;

const fn seconds_to_micros(seconds: u64) -> u64 {
    seconds * 1_000_000
}

const fn millis_to_micros(millis: u64) -> u64 {
    millis * 1_000
}

/// An instant in monotonic time as provided by a `CLOCK_MONOTONIC` like clock
///
/// Internally, this uses microsecond (micros) sized values to track, giving 584942417355.07202148
/// years until overflow. Internally, systemd uses the same layout for it's time values.
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

            pub const fn from_millis(millis: u64) -> Self {
                Self {
                    micros: millis_to_micros(millis),
                }
            }

            pub const fn from_seconds(seconds: u32) -> Self {
                Self {
                    micros: seconds_to_micros(seconds as u64),
                }
            }
        }

        impl Sub for $time_type {
            type Output = Duration;

            fn sub(self, rhs: Self) -> Self::Output {
                Duration::from_micros(self.micros - rhs.micros)
            }
        }

        impl Add<time::Duration> for $time_type {
            type Output = Self;

            fn add(self, other: time::Duration) -> Self {
                let mo: u64 = other.as_micros().try_into().unwrap();
                Self {
                    micros: self.micros + mo,
                }
            }
        }

        impl Add<Duration> for $time_type {
            type Output = Self;

            fn add(self, other: Duration) -> Self {
                Self {
                    micros: self.micros + other.micros,
                }
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

impl Add for Duration {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            micros: self.micros + rhs.micros,
        }
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            micros: self.micros - rhs.micros,
        }
    }
}

// TODO: make this `const` once duration_consts_2 is stable
fn duration_from_micros(micros: u64) -> time::Duration {
    let secs = micros / 1_000_000;
    let sub_micros = (micros % 1_000_000) as u32;
    let sub_nsec = sub_micros * 1000;
    time::Duration::new(secs, sub_nsec)
}

const fn micros_from_duration(duration: time::Duration) -> u64 {
    let sub_micros = (duration.subsec_nanos() / 1000) as u64;
    duration.as_secs() * 1_000_000 + sub_micros
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
    pub fn from_core_duration(ctd: core::time::Duration) -> Self {
        Self::from_micros(micros_from_duration(ctd))
    }
}
