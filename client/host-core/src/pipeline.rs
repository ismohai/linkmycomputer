use thiserror::Error;

use crate::capture::dxgi::CaptureDescriptor;
use crate::config::profile::{Codec, RuntimeProfile};
use crate::encode::nvenc::EncoderDescriptor;
use crate::transport::webrtc::TransportDescriptor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostCapability {
    pub max_width: u16,
    pub max_height: u16,
    pub max_fps: u16,
    pub codecs: Vec<Codec>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineDescriptor {
    pub capture: CaptureDescriptor,
    pub encoder: EncoderDescriptor,
    pub transport: TransportDescriptor,
}

pub fn build_locked_pipeline(
    profile: &RuntimeProfile,
    capability: &HostCapability,
) -> Result<PipelineDescriptor, PipelineError> {
    if profile.width > capability.max_width || profile.height > capability.max_height {
        return Err(PipelineError::ResolutionUnsupported {
            requested_width: profile.width,
            requested_height: profile.height,
            max_width: capability.max_width,
            max_height: capability.max_height,
        });
    }

    if profile.fps > capability.max_fps {
        return Err(PipelineError::FpsUnsupported {
            requested: profile.fps,
            max: capability.max_fps,
        });
    }

    if !capability.codecs.contains(&profile.codec) {
        return Err(PipelineError::CodecUnsupported(profile.codec));
    }

    Ok(PipelineDescriptor {
        capture: CaptureDescriptor::from_profile(profile),
        encoder: EncoderDescriptor::from_profile(profile),
        transport: TransportDescriptor::lan_low_latency(),
    })
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PipelineError {
    #[error("requested codec is unsupported: {0:?}")]
    CodecUnsupported(Codec),
    #[error("requested fps {requested} exceeds host maximum {max}")]
    FpsUnsupported { requested: u16, max: u16 },
    #[error(
        "requested resolution {requested_width}x{requested_height} exceeds host maximum {max_width}x{max_height}"
    )]
    ResolutionUnsupported {
        requested_width: u16,
        requested_height: u16,
        max_width: u16,
        max_height: u16,
    },
}
