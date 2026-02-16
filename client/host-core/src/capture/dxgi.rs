use crate::config::profile::RuntimeProfile;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaptureDescriptor {
    pub width: u16,
    pub height: u16,
    pub fps: u16,
    pub source_hint: String,
}

impl CaptureDescriptor {
    pub fn from_profile(profile: &RuntimeProfile) -> Self {
        Self {
            width: profile.width,
            height: profile.height,
            fps: profile.fps,
            source_hint: "mumu-window".to_string(),
        }
    }
}
