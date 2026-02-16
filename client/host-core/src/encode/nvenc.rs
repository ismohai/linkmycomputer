use crate::config::profile::{Codec, RuntimeProfile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EncoderDescriptor {
    pub codec: Codec,
    pub target_bitrate_kbps: u32,
    pub low_latency: bool,
    pub preset: String,
}

impl EncoderDescriptor {
    pub fn from_profile(profile: &RuntimeProfile) -> Self {
        let preset = match profile.fps {
            120 | 144 => "p1_low_latency_hq",
            _ => "p3_low_latency",
        };

        Self {
            codec: profile.codec,
            target_bitrate_kbps: profile.target_bitrate_kbps,
            low_latency: true,
            preset: preset.to_string(),
        }
    }
}
