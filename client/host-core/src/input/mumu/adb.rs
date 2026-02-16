#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdbDeviceState {
    Device,
    Offline,
    Unauthorized,
    Unknown(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdbDevice {
    pub serial: String,
    pub state: AdbDeviceState,
}

pub fn parse_adb_devices(raw: &str) -> Vec<AdbDevice> {
    raw.lines()
        .skip(1)
        .filter_map(|line| {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                return None;
            }

            let mut parts = trimmed.split_whitespace();
            let serial = parts.next()?;
            let state = parts.next()?;

            Some(AdbDevice {
                serial: serial.to_string(),
                state: parse_state(state),
            })
        })
        .collect()
}

pub fn find_mumu_candidate(devices: &[AdbDevice]) -> Option<AdbDevice> {
    devices
        .iter()
        .filter(|d| d.state == AdbDeviceState::Device)
        .max_by_key(|d| candidate_rank(&d.serial))
        .cloned()
}

fn parse_state(raw: &str) -> AdbDeviceState {
    match raw {
        "device" => AdbDeviceState::Device,
        "offline" => AdbDeviceState::Offline,
        "unauthorized" => AdbDeviceState::Unauthorized,
        other => AdbDeviceState::Unknown(other.to_string()),
    }
}

fn candidate_rank(serial: &str) -> u8 {
    if serial.starts_with("127.0.0.1:") {
        3
    } else if serial.ends_with(":7555") {
        2
    } else {
        1
    }
}
