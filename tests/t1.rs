#[cfg(feature = "std")]
#[test]
fn realtime_systemtime_round_trip() {
    let us = 10_000_001;
    let m_rt: microtime::RealTime =
        microtime::RealTime::ZERO + microtime::Duration::from_micros(us);
    let s_rt: std::time::SystemTime = m_rt.try_into().unwrap();
    let m_rt2: microtime::RealTime = s_rt.try_into().unwrap();

    assert_eq!(us, (m_rt2 - microtime::RealTime::ZERO).as_micros());
}

#[cfg(feature = "std")]
#[test]
fn realtime_rejects_pre_epoch_system_time() {
    let before_epoch = std::time::UNIX_EPOCH - std::time::Duration::from_micros(1);

    assert_eq!(
        microtime::RealTime::try_from(before_epoch),
        Err(microtime::TimeRangeError)
    );
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

#[test]
fn checked_arithmetic_reports_values_outside_the_range() {
    let one = microtime::Duration::from_micros(1);

    assert_eq!(microtime::Duration::checked_from_seconds(u64::MAX), None);
    assert_eq!(
        microtime::Duration::checked_from_milliseconds(u64::MAX),
        None
    );
    assert_eq!(
        microtime::Duration::from_micros(u64::MAX).checked_add(one),
        None
    );
    assert_eq!(microtime::Duration::ZERO.checked_sub(one), None);
    assert_eq!(
        microtime::MonotonicTime::ZERO
            .checked_duration_since(microtime::MonotonicTime::from_micros(1)),
        None
    );
    assert_eq!(
        microtime::MonotonicTime::checked_from_millis(u64::MAX),
        None
    );
}

#[test]
#[should_panic(expected = "seconds do not fit in microseconds")]
fn duration_seconds_overflow_panics_in_all_profiles() {
    let _ = microtime::Duration::from_seconds(u64::MAX);
}

#[test]
#[should_panic(expected = "duration addition overflowed")]
fn duration_add_overflow_panics_in_all_profiles() {
    let _ = microtime::Duration::from_micros(u64::MAX) + microtime::Duration::from_micros(1);
}

#[test]
#[should_panic(expected = "cannot subtract a later timestamp")]
fn timestamp_subtraction_underflow_panics_in_all_profiles() {
    let _ = microtime::MonotonicTime::ZERO - microtime::MonotonicTime::from_micros(1);
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
