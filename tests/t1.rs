extern crate microtime;

use std::convert::From;

#[test]
fn realtime_systemtime_round_trip() {
    let us = 10_000_001;
    let m_rt = microtime::RealTime::from_micros(us);
    let s_rt = std::time::SystemTime::from(m_rt);
    let m_rt2 = microtime::RealTime::from(s_rt);

    assert_eq!(us, m_rt2.as_micros());
}

#[test]
fn monotonic_duration_round_trip() {
    let us = 10_000_001;
    let m_t = microtime::MonotonicTime::from_micros(us);
    let s_t = std::time::Duration::from(m_t);
    let m_t2 = microtime::MonotonicTime::from(s_t);

    assert_eq!(us, m_t2.as_micros());
}
