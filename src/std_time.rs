use super::*;
use std::time;

fn system_time_from_realtime_micros(micros: u64) -> time::SystemTime {
    let d = duration_from_micros(micros);
    time::UNIX_EPOCH + d
}

fn micros_from_system_time(ts: time::SystemTime) -> u64 {
    let d = ts.duration_since(time::UNIX_EPOCH).unwrap();
    u64::try_from(d.as_micros()).unwrap()
}

impl From<time::SystemTime> for RealTime {
    fn from(s: time::SystemTime) -> Self {
        Self {
            micros: micros_from_system_time(s),
        }
    }
}

impl From<RealTime> for time::SystemTime {
    fn from(s: RealTime) -> time::SystemTime {
        system_time_from_realtime_micros(s.micros)
    }
}
