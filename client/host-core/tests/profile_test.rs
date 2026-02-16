use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};

#[test]
fn turbo_lock_profile_uses_requested_caps() {
    let profile = RuntimeProfile::new(2460, 1080, 144, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect("profile should be valid");

    assert_eq!(profile.width, 2460);
    assert_eq!(profile.height, 1080);
    assert_eq!(profile.fps, 144);
    assert_eq!(profile.target_bitrate_kbps, 80_000);
    assert_eq!(profile.lock_policy, LockPolicy::TurboLock);
}

#[test]
fn invalid_fps_reports_supported_presets() {
    let err = RuntimeProfile::new(2460, 1080, 75, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect_err("75fps must be rejected");

    assert!(err
        .to_string()
        .contains("supported fps presets: 60, 90, 120, 144"));
}

#[test]
fn invalid_resolution_reports_fallbacks() {
    let err = RuntimeProfile::new(2100, 1000, 120, 50_000, Codec::H264, LockPolicy::TurboLock)
        .expect_err("unknown resolution must be rejected");

    assert!(err
        .to_string()
        .contains("supported resolutions: 1280x720, 1600x900, 1920x1080, 2460x1080"));
}
