#[cfg(feature = "std")]
#[test]
fn realtime_systemtime_round_trip() {
    let us = 10_000_001;
    let m_rt: microtime::RealTime =
        microtime::RealTime::ZERO + microtime::Duration::from_micros(us);
    let s_rt: std::time::SystemTime = m_rt.into();
    let m_rt2: microtime::RealTime = s_rt.into();

    assert_eq!(us, (m_rt2 - microtime::RealTime::ZERO).as_micros());
}

#[test]
fn monotonic_duration_round_trip() {
    let us = 10_000_001;
    let m_t: microtime::MonotonicTime =
        microtime::MonotonicTime::ZERO + microtime::Duration::from_micros(us);
    let s_t: core::time::Duration = m_t.to_core_duration();
    let m_t2 = microtime::MonotonicTime::from_core_duration(s_t);

    assert_eq!(us, m_t2.as_micros());
}

// `Add` isn't const
/*
const fn dur_1sec_10micro() -> microtime::Duration {
    microtime::Duration::from_seconds(1) + microtime::Duration::from_micros(10)
}

#[test]
fn const_check() {
    assert_eq!(
        dur_1sec_10micro(),
        microtime::Duration::from_seconds(1) + microtime::Duration::from_micros(10)
    );
}
*/
