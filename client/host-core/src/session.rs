use thiserror::Error;

use crate::config::profile::RuntimeProfile;
use crate::pipeline::{build_locked_pipeline, HostCapability, PipelineDescriptor, PipelineError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionState {
    Idle,
    Starting,
    Running,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SessionStarted {
    pub pipeline: PipelineDescriptor,
}

#[derive(Debug, Default)]
pub struct SessionManager {
    state: SessionState,
    active: Option<SessionStarted>,
}

impl SessionManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn state(&self) -> SessionState {
        self.state
    }

    pub fn start(
        &mut self,
        profile: RuntimeProfile,
        capability: HostCapability,
    ) -> Result<SessionStarted, SessionError> {
        if self.state != SessionState::Idle {
            return Err(SessionError::InvalidTransition(
                self.state,
                SessionState::Starting,
            ));
        }

        self.state = SessionState::Starting;

        let pipeline = match build_locked_pipeline(&profile, &capability) {
            Ok(pipeline) => pipeline,
            Err(err) => {
                self.state = SessionState::Idle;
                return Err(SessionError::Pipeline(err));
            }
        };
        let started = SessionStarted { pipeline };

        self.active = Some(started.clone());
        self.state = SessionState::Running;

        Ok(started)
    }

    pub fn stop(&mut self) {
        self.active = None;
        self.state = SessionState::Idle;
    }
}

impl Default for SessionState {
    fn default() -> Self {
        SessionState::Idle
    }
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("session state transition from {0:?} to {1:?} is not allowed")]
    InvalidTransition(SessionState, SessionState),
    #[error("session pipeline validation failed: {0}")]
    Pipeline(PipelineError),
}
