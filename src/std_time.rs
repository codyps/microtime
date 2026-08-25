use super::*;
use std::time;

fn system_time_from_realtime_micros(micros: u64) -> TimeResult<time::SystemTime> {
    let d = duration_from_micros(micros);
    time::UNIX_EPOCH.checked_add(d).ok_or(TimeRangeError)
}

fn micros_from_system_time(ts: time::SystemTime) -> TimeResult<u64> {
    let duration = ts
        .duration_since(time::UNIX_EPOCH)
        .map_err(|_| TimeRangeError)?;
    micros_from_duration(duration)
}

impl TryFrom<time::SystemTime> for RealTime {
    type Error = TimeRangeError;

    fn try_from(system_time: time::SystemTime) -> TimeResult<Self> {
        Ok(Self::from_micros(micros_from_system_time(system_time)?))
    }
}

impl TryFrom<RealTime> for time::SystemTime {
    type Error = TimeRangeError;

    fn try_from(real_time: RealTime) -> TimeResult<Self> {
        system_time_from_realtime_micros(real_time.micros)
    }
}
