use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const SUPPORTED_FPS: [u16; 4] = [60, 90, 120, 144];
pub const SUPPORTED_RESOLUTIONS: [(u16, u16); 4] =
    [(1280, 720), (1600, 900), (1920, 1080), (2460, 1080)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Codec {
    H264,
    Hevc,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockPolicy {
    TurboLock,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeProfile {
    pub width: u16,
    pub height: u16,
    pub fps: u16,
    pub target_bitrate_kbps: u32,
    pub codec: Codec,
    pub lock_policy: LockPolicy,
}

impl RuntimeProfile {
    pub fn new(
        width: u16,
        height: u16,
        fps: u16,
        target_bitrate_kbps: u32,
        codec: Codec,
        lock_policy: LockPolicy,
    ) -> Result<Self, ProfileError> {
        if !SUPPORTED_FPS.contains(&fps) {
            return Err(ProfileError::UnsupportedFps(fps));
        }

        if !SUPPORTED_RESOLUTIONS.contains(&(width, height)) {
            return Err(ProfileError::UnsupportedResolution(width, height));
        }

        if target_bitrate_kbps == 0 {
            return Err(ProfileError::InvalidBitrate(target_bitrate_kbps));
        }

        Ok(Self {
            width,
            height,
            fps,
            target_bitrate_kbps,
            codec,
            lock_policy,
        })
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ProfileError {
    #[error("unsupported fps preset {0}; supported fps presets: 60, 90, 120, 144")]
    UnsupportedFps(u16),
    #[error(
        "unsupported resolution {0}x{1}; supported resolutions: 1280x720, 1600x900, 1920x1080, 2460x1080"
    )]
    UnsupportedResolution(u16, u16),
    #[error("target bitrate must be > 0 kbps")]
    InvalidBitrate(u32),
}
