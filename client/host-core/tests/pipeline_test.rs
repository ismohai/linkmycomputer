use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::pipeline::{build_locked_pipeline, HostCapability, PipelineError};

#[test]
fn pipeline_builds_when_capability_matches_locked_profile() {
    let profile = RuntimeProfile::new(2460, 1080, 144, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect("valid profile");

    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 144,
        codecs: vec![Codec::Hevc, Codec::H264],
    };

    let pipeline = build_locked_pipeline(&profile, &capability).expect("pipeline");
    assert_eq!(pipeline.encoder.codec, Codec::Hevc);
    assert!(pipeline.encoder.low_latency);
    assert!(pipeline.transport.force_lan_mode);
}

#[test]
fn pipeline_rejects_profile_when_codec_is_not_supported() {
    let profile = RuntimeProfile::new(2460, 1080, 144, 80_000, Codec::Hevc, LockPolicy::TurboLock)
        .expect("valid profile");
    let capability = HostCapability {
        max_width: 3840,
        max_height: 2160,
        max_fps: 240,
        codecs: vec![Codec::H264],
    };

    let err = build_locked_pipeline(&profile, &capability).expect_err("unsupported codec");
    assert!(matches!(err, PipelineError::CodecUnsupported(Codec::Hevc)));
}

#[test]
fn pipeline_rejects_profile_when_fps_exceeds_capability() {
    let profile = RuntimeProfile::new(1920, 1080, 144, 80_000, Codec::H264, LockPolicy::TurboLock)
        .expect("valid profile");
    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 120,
        codecs: vec![Codec::H264],
    };

    let err = build_locked_pipeline(&profile, &capability).expect_err("fps should be rejected");
    assert!(matches!(
        err,
        PipelineError::FpsUnsupported {
            requested: 144,
            max: 120
        }
    ));
}
