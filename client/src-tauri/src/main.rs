use std::sync::Mutex;

use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::pipeline::HostCapability;
use host_core::session::{SessionManager, SessionState};
use serde::{Deserialize, Serialize};

struct HostState {
    session: Mutex<SessionManager>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SessionPayload {
    fps: u16,
    resolution: String,
    bitrate_kbps: u32,
    lock_policy: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionStatusPayload {
    state: SessionStateValue,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum SessionStateValue {
    Idle,
    Starting,
    Running,
}

#[tauri::command]
fn start_locked_session(
    payload: SessionPayload,
    state: tauri::State<HostState>,
) -> Result<SessionPayload, String> {
    if payload.lock_policy != "turbo_lock" {
        return Err("锁定策略必须为 turbo_lock".to_string());
    }

    let (width, height) = parse_resolution(&payload.resolution)?;
    let profile = RuntimeProfile::new(
        width,
        height,
        payload.fps,
        payload.bitrate_kbps,
        Codec::Hevc,
        LockPolicy::TurboLock,
    )
    .map_err(|err| err.to_string())?;

    let capability = HostCapability {
        max_width: 2560,
        max_height: 1440,
        max_fps: 144,
        codecs: vec![Codec::Hevc, Codec::H264],
    };

    let mut manager = state
        .session
        .lock()
        .map_err(|_| "会话管理器加锁失败".to_string())?;
    manager
        .start(profile, capability)
        .map_err(|err| err.to_string())?;

    Ok(payload)
}

#[tauri::command]
fn stop_session(state: tauri::State<HostState>) -> Result<(), String> {
    let mut manager = state
        .session
        .lock()
        .map_err(|_| "会话管理器加锁失败".to_string())?;
    manager.stop();
    Ok(())
}

#[tauri::command]
fn session_status(state: tauri::State<HostState>) -> Result<SessionStatusPayload, String> {
    let manager = state
        .session
        .lock()
        .map_err(|_| "会话管理器加锁失败".to_string())?;

    let value = match manager.state() {
        SessionState::Idle => SessionStateValue::Idle,
        SessionState::Starting => SessionStateValue::Starting,
        SessionState::Running => SessionStateValue::Running,
    };

    Ok(SessionStatusPayload { state: value })
}

fn parse_resolution(value: &str) -> Result<(u16, u16), String> {
    let (w, h) = value.split_once('x').ok_or("分辨率格式必须为 宽x高")?;
    let width = w.parse::<u16>().map_err(|err| format!("宽度无效: {err}"))?;
    let height = h.parse::<u16>().map_err(|err| format!("高度无效: {err}"))?;
    Ok((width, height))
}

fn main() {
    tauri::Builder::default()
        .manage(HostState {
            session: Mutex::new(SessionManager::new()),
        })
        .invoke_handler(tauri::generate_handler![
            start_locked_session,
            stop_session,
            session_status
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
