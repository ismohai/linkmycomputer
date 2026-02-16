use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::pipeline::HostCapability;
use host_core::session::{SessionError, SessionManager, SessionState};

#[test]
fn start_session_moves_state_to_running_when_capability_matches() {
    let profile = RuntimeProfile::new(2460, 1080, 144, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect("profile");
    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 144,
        codecs: vec![Codec::Hevc, Codec::H264],
    };
    let mut manager = SessionManager::new();

    let started = manager.start(profile, capability).expect("start succeeds");

    assert_eq!(manager.state(), SessionState::Running);
    assert_eq!(started.pipeline.capture.fps, 144);
}

#[test]
fn start_session_fails_and_keeps_idle_state_when_capability_is_insufficient() {
    let profile = RuntimeProfile::new(2460, 1080, 144, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect("profile");
    let capability = HostCapability {
        max_width: 1920,
        max_height: 1080,
        max_fps: 120,
        codecs: vec![Codec::H264],
    };
    let mut manager = SessionManager::new();

    let err = manager
        .start(profile, capability)
        .expect_err("insufficient capability");

    assert!(matches!(err, SessionError::Pipeline(_)));
    assert_eq!(manager.state(), SessionState::Idle);
}

#[test]
fn stop_session_moves_running_state_back_to_idle() {
    let profile = RuntimeProfile::new(1920, 1080, 120, 60_000, Codec::H264, LockPolicy::TurboLock)
        .expect("profile");
    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 144,
        codecs: vec![Codec::H264],
    };
    let mut manager = SessionManager::new();

    manager.start(profile, capability).expect("start succeeds");
    manager.stop();

    assert_eq!(manager.state(), SessionState::Idle);
}
