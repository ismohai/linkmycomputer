use std::process::Command;

use thiserror::Error;

use crate::input::mumu::adb::{find_mumu_candidate, parse_adb_devices, AdbDevice};
use crate::input::mumu::minitouch::{MinitouchBuilder, TouchPoint};
use crate::protocol::control::{PointerAction, PointerEvent};

#[derive(Debug, Clone, Copy)]
pub struct MumuBridge {
    width: u32,
    height: u32,
}

impl MumuBridge {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn discover_serial_from_adb_output(&self, raw: &str) -> Result<String, MumuBridgeError> {
        let devices = parse_adb_devices(raw);
        let device = find_mumu_candidate(&devices).ok_or(MumuBridgeError::NoDeviceFound)?;
        Ok(device.serial)
    }

    pub fn discover_serial_via_adb(&self, adb_path: &str) -> Result<String, MumuBridgeError> {
        let devices = self.query_adb_devices(adb_path)?;
        let device = find_mumu_candidate(&devices).ok_or(MumuBridgeError::NoDeviceFound)?;
        Ok(device.serial)
    }

    pub fn query_adb_devices(&self, adb_path: &str) -> Result<Vec<AdbDevice>, MumuBridgeError> {
        let output = Command::new(adb_path)
            .arg("devices")
            .output()
            .map_err(MumuBridgeError::AdbExecution)?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(MumuBridgeError::AdbFailed(stderr));
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_adb_devices(&stdout))
    }

    pub fn build_minitouch_payload(
        &self,
        events: &[PointerEvent],
    ) -> Result<String, MumuBridgeError> {
        if events.is_empty() {
            return Err(MumuBridgeError::EmptyEventBatch);
        }

        let mut builder = MinitouchBuilder::default();
        for event in events {
            let point = TouchPoint::new(
                event.pointer_id,
                self.to_pixel(event.x, self.width),
                self.to_pixel(event.y, self.height),
                self.to_pressure(event.pressure),
            );

            builder = match event.action {
                PointerAction::Down => builder.down(point),
                PointerAction::Move => builder.move_to(point),
                PointerAction::Up | PointerAction::Cancel => builder.up(event.pointer_id),
            };
        }

        Ok(builder.commit().to_string())
    }

    fn to_pixel(&self, normalized: f32, max: u32) -> u32 {
        let safe = normalized.clamp(0.0, 1.0);
        (safe * max as f32).round() as u32
    }

    fn to_pressure(&self, normalized: f32) -> u32 {
        let safe = normalized.clamp(0.0, 1.0);
        (safe * 100.0).round() as u32
    }
}

#[derive(Debug, Error)]
pub enum MumuBridgeError {
    #[error("no MuMu-compatible ADB device found")]
    NoDeviceFound,
    #[error("touch event batch is empty")]
    EmptyEventBatch,
    #[error("failed to execute adb: {0}")]
    AdbExecution(std::io::Error),
    #[error("adb command failed: {0}")]
    AdbFailed(String),
}
