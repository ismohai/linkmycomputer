use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PointerAction {
    Down,
    Move,
    Up,
    Cancel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PointerEvent {
    pub pointer_id: u8,
    pub action: PointerAction,
    pub x: f32,
    pub y: f32,
    pub pressure: f32,
    pub timestamp_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TouchEnvelope {
    pub frame_id: u64,
    pub events: Vec<PointerEvent>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload", rename_all = "snake_case")]
pub enum ControlFrame {
    Touch(TouchEnvelope),
    Ping { timestamp_ms: u64 },
}

impl ControlFrame {
    pub fn to_wire_bytes(&self) -> Result<Vec<u8>, ControlCodecError> {
        serde_json::to_vec(self).map_err(ControlCodecError::Serialize)
    }

    pub fn from_wire_bytes(payload: &[u8]) -> Result<Self, ControlCodecError> {
        if payload.is_empty() {
            return Err(ControlCodecError::EmptyPayload);
        }

        let frame: ControlFrame =
            serde_json::from_slice(payload).map_err(ControlCodecError::Deserialize)?;
        frame.validate()?;
        Ok(frame)
    }

    fn validate(&self) -> Result<(), ControlCodecError> {
        const MAX_EVENTS_PER_FRAME: usize = 32;

        if let ControlFrame::Touch(touch) = self {
            if touch.events.is_empty() {
                return Err(ControlCodecError::EmptyTouchFrame);
            }

            if touch.events.len() > MAX_EVENTS_PER_FRAME {
                return Err(ControlCodecError::TooManyEvents(touch.events.len()));
            }

            for event in &touch.events {
                if !event.x.is_finite()
                    || !event.y.is_finite()
                    || !(0.0..=1.0).contains(&event.x)
                    || !(0.0..=1.0).contains(&event.y)
                {
                    return Err(ControlCodecError::InvalidCoordinateRange);
                }

                if !event.pressure.is_finite() || !(0.0..=1.0).contains(&event.pressure) {
                    return Err(ControlCodecError::InvalidPressureRange);
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ControlCodecError {
    #[error("control payload is empty")]
    EmptyPayload,
    #[error("failed to serialize control frame: {0}")]
    Serialize(serde_json::Error),
    #[error("failed to decode control frame: {0}")]
    Deserialize(serde_json::Error),
    #[error("touch frame cannot be empty")]
    EmptyTouchFrame,
    #[error("touch frame exceeds max event count: {0}")]
    TooManyEvents(usize),
    #[error("touch event x/y must be finite and within [0.0, 1.0]")]
    InvalidCoordinateRange,
    #[error("touch event pressure must be finite and within [0.0, 1.0]")]
    InvalidPressureRange,
}
