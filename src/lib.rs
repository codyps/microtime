use std::convert::TryFrom;
use std::time;

fn usec_from_duration(duration: time::Duration) -> u64 {
    let sub_usecs = (duration.subsec_nanos() / 1000) as u64;
    duration.as_secs() * 1_000_000 + sub_usecs
}

fn duration_from_usec(usec: u64) -> time::Duration {
    let secs = usec / 1_000_000;
    let sub_usec = (usec % 1_000_000) as u32;
    let sub_nsec = sub_usec * 1000;
    time::Duration::new(secs, sub_nsec)
}

fn system_time_from_realtime_usec(usec: u64) -> time::SystemTime {
    let d = duration_from_usec(usec);
    time::UNIX_EPOCH + d
}

fn usec_from_system_time(ts: time::SystemTime) -> u64
{
    let d = ts.duration_since(time::UNIX_EPOCH).unwrap();
    u64::try_from(d.as_micros()).unwrap()
}

/// An instant in monotonic time as provided by a `CLOCK_MONOTONIC` like clock
///
/// Internally, this uses microsecond (usec) sized values to track, giving 584942417355.07202148
/// years until overflow. Internally, systemd uses the same formatting for it's time values.
#[derive(Debug,Eq,PartialEq,PartialOrd,Ord,Clone,Copy)]
pub struct MonotonicTime {
    usecs: u64,
}

impl MonotonicTime {
    /// Create a timestamp from microseconds
    pub fn from_micros(usecs: u64) -> Self {
        MonotonicTime { usecs: usecs }
    }

    /// Return the entire timestamp converted to microseconds
    pub fn as_micros(&self) -> u64 {
        self.usecs
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
#[derive(Debug,Eq,PartialEq,PartialOrd,Ord,Clone,Copy)]
pub struct RealTime {
    usecs: u64,
}

impl RealTime {
    pub fn from_micros(micros: u64) -> Self
    {
        Self { usecs: micros }
    }

    pub fn as_micros(&self) -> u64
    {
        self.usecs
    }
}

impl From<time::SystemTime> for RealTime {
    fn from(s: time::SystemTime) -> Self {
        Self::from_micros(usec_from_system_time(s))
    }
}

impl From<RealTime> for time::SystemTime {
    fn from(s: RealTime) -> time::SystemTime {
        system_time_from_realtime_usec(s.as_micros())
    }
}
