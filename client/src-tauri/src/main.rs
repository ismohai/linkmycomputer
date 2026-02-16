use std::collections::HashMap;
use std::net::{SocketAddr, UdpSocket};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use host_core::config::profile::{Codec, LockPolicy, RuntimeProfile};
use host_core::input::mumu::bridge::MumuBridge;
use host_core::pipeline::HostCapability;
use host_core::protocol::control::PointerAction;
use host_core::session::{SessionManager, SessionState};
use serde::{Deserialize, Serialize};

struct HostState {
    session: Mutex<SessionManager>,
    connection: Mutex<Option<LanDevice>>,
    touch_runtime: Arc<Mutex<TouchRuntime>>,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct LanDevice {
    id: String,
    name: String,
    ip: String,
    control_port: u16,
    version: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct LanConnectionStatusPayload {
    connected: bool,
    device: Option<LanDevice>,
    message: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
enum SessionStateValue {
    Idle,
    Starting,
    Running,
}

#[derive(Debug)]
struct TouchRuntime {
    connected_device_ip: Option<String>,
    mumu_serial: Option<String>,
    down_points: HashMap<u8, (u32, u32)>,
    last_points: HashMap<u8, (u32, u32)>,
    target_width: u32,
    target_height: u32,
}

impl Default for TouchRuntime {
    fn default() -> Self {
        Self {
            connected_device_ip: None,
            mumu_serial: None,
            down_points: HashMap::new(),
            last_points: HashMap::new(),
            target_width: 2460,
            target_height: 1080,
        }
    }
}

impl TouchRuntime {
    fn clear_connection(&mut self) {
        self.connected_device_ip = None;
        self.mumu_serial = None;
        self.down_points.clear();
        self.last_points.clear();
    }
}

#[derive(Debug)]
struct TouchEventPacket {
    pointer_id: u8,
    action: PointerAction,
    x: f32,
    y: f32,
    pressure: f32,
}

#[derive(Debug)]
enum AdbTouchCommand {
    Tap {
        x: u32,
        y: u32,
    },
    Swipe {
        start_x: u32,
        start_y: u32,
        end_x: u32,
        end_y: u32,
        duration_ms: u32,
    },
}

const DISCOVERY_PORT: u16 = 42042;
const HOST_TOUCH_PORT: u16 = 42044;
const DISCOVERY_TIMEOUT_MS: u64 = 1_300;
const REQUEST_TIMEOUT_MS: u64 = 5_000;
const PING_TIMEOUT_MS: u64 = 900;

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

    if let Ok(mut runtime) = state.touch_runtime.lock() {
        runtime.target_width = width as u32;
        runtime.target_height = height as u32;
    }

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

#[tauri::command]
fn scan_lan_devices() -> Result<Vec<LanDevice>, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| format!("绑定扫描端口失败: {err}"))?;
    socket
        .set_broadcast(true)
        .map_err(|err| format!("启用广播失败: {err}"))?;
    socket
        .set_read_timeout(Some(Duration::from_millis(250)))
        .map_err(|err| format!("设置扫描超时失败: {err}"))?;

    let local_port = socket
        .local_addr()
        .map_err(|err| format!("读取本地端口失败: {err}"))?
        .port();

    let packet = format!("LMC_DISCOVER|{}|{}", desktop_name(), local_port);
    socket
        .send_to(packet.as_bytes(), ("255.255.255.255", DISCOVERY_PORT))
        .map_err(|err| format!("发送扫描广播失败: {err}"))?;

    let begin = Instant::now();
    let mut buffer = [0_u8; 1024];
    let mut devices = HashMap::<String, LanDevice>::new();

    while begin.elapsed() < Duration::from_millis(DISCOVERY_TIMEOUT_MS) {
        match socket.recv_from(&mut buffer) {
            Ok((length, from)) => {
                let text = String::from_utf8_lossy(&buffer[..length]).to_string();
                if let Some(device) = parse_discovery_response(&text, from) {
                    devices.insert(device.id.clone(), device);
                }
            }
            Err(err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut => {}
            Err(err) => return Err(format!("接收扫描结果失败: {err}")),
        }
    }

    let mut list = devices.into_values().collect::<Vec<_>>();
    list.sort_by(|a, b| a.name.cmp(&b.name).then(a.ip.cmp(&b.ip)));
    Ok(list)
}

#[tauri::command]
fn request_device_connection(
    device: LanDevice,
    state: tauri::State<HostState>,
) -> Result<LanConnectionStatusPayload, String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| format!("创建连接通道失败: {err}"))?;
    socket
        .set_read_timeout(Some(Duration::from_millis(450)))
        .map_err(|err| format!("设置连接超时失败: {err}"))?;

    let message = format!("LMC_CONNECT_REQUEST|{}|{}", desktop_name(), HOST_TOUCH_PORT);
    let target = format!("{}:{}", device.ip, device.control_port);
    socket
        .send_to(message.as_bytes(), &target)
        .map_err(|err| format!("发送连接请求失败: {err}"))?;

    let begin = Instant::now();
    let mut buffer = [0_u8; 1024];

    while begin.elapsed() < Duration::from_millis(REQUEST_TIMEOUT_MS) {
        match socket.recv_from(&mut buffer) {
            Ok((length, from)) => {
                if from.ip().to_string() != device.ip {
                    continue;
                }

                let text = String::from_utf8_lossy(&buffer[..length])
                    .trim()
                    .to_string();
                if text.starts_with("LMC_CONNECT_ACCEPT") {
                    {
                        let mut current = state
                            .connection
                            .lock()
                            .map_err(|_| "连接状态加锁失败".to_string())?;
                        *current = Some(device.clone());
                    }

                    let mut runtime = state
                        .touch_runtime
                        .lock()
                        .map_err(|_| "触控运行态加锁失败".to_string())?;
                    runtime.connected_device_ip = Some(device.ip.clone());
                    runtime.mumu_serial = None;
                    runtime.down_points.clear();
                    runtime.last_points.clear();

                    let bridge_status = match ensure_mumu_serial(&mut runtime) {
                        Ok(serial) => format!("手机已确认连接，可直接控制（MuMu: {serial}）"),
                        Err(err) => {
                            format!("手机已确认连接，但MuMu控制通道未就绪：{err}")
                        }
                    };

                    return Ok(LanConnectionStatusPayload {
                        connected: true,
                        device: Some(device),
                        message: bridge_status,
                    });
                }

                if text.starts_with("LMC_CONNECT_REJECT") {
                    return Ok(LanConnectionStatusPayload {
                        connected: false,
                        device: None,
                        message: "手机拒绝了连接请求。".to_string(),
                    });
                }
            }
            Err(err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut => {}
            Err(err) => return Err(format!("等待手机响应失败: {err}")),
        }
    }

    Ok(LanConnectionStatusPayload {
        connected: false,
        device: None,
        message: "手机未在超时时间内响应，请重试。".to_string(),
    })
}

#[tauri::command]
fn disconnect_device(state: tauri::State<HostState>) -> Result<LanConnectionStatusPayload, String> {
    let device = {
        let mut current = state
            .connection
            .lock()
            .map_err(|_| "连接状态加锁失败".to_string())?;
        current.take()
    };

    if let Ok(mut runtime) = state.touch_runtime.lock() {
        runtime.clear_connection();
    }

    if let Some(device) = device {
        let _ = send_udp_message(&device.ip, device.control_port, "LMC_DISCONNECT|HOST");
        return Ok(LanConnectionStatusPayload {
            connected: false,
            device: None,
            message: "已从电脑端断开手机连接。".to_string(),
        });
    }

    Ok(LanConnectionStatusPayload {
        connected: false,
        device: None,
        message: "当前没有已连接手机。".to_string(),
    })
}

#[tauri::command]
fn connection_status(state: tauri::State<HostState>) -> Result<LanConnectionStatusPayload, String> {
    let device = {
        state
            .connection
            .lock()
            .map_err(|_| "连接状态加锁失败".to_string())?
            .clone()
    };

    if let Some(device) = device {
        if ping_device(&device).is_ok() {
            return Ok(LanConnectionStatusPayload {
                connected: true,
                device: Some(device.clone()),
                message: format!(
                    "已连接手机：{}（{}），可在手机控制区直接操作",
                    device.name, device.ip
                ),
            });
        }

        {
            let mut current = state
                .connection
                .lock()
                .map_err(|_| "连接状态加锁失败".to_string())?;
            *current = None;
        }

        if let Ok(mut runtime) = state.touch_runtime.lock() {
            runtime.clear_connection();
        }

        return Ok(LanConnectionStatusPayload {
            connected: false,
            device: None,
            message: "手机已离线，连接已自动断开。".to_string(),
        });
    }

    Ok(LanConnectionStatusPayload {
        connected: false,
        device: None,
        message: "尚未连接手机。".to_string(),
    })
}

fn parse_resolution(value: &str) -> Result<(u16, u16), String> {
    let (w, h) = value.split_once('x').ok_or("分辨率格式必须为 宽x高")?;
    let width = w.parse::<u16>().map_err(|err| format!("宽度无效: {err}"))?;
    let height = h.parse::<u16>().map_err(|err| format!("高度无效: {err}"))?;
    Ok((width, height))
}

fn parse_discovery_response(message: &str, from: SocketAddr) -> Option<LanDevice> {
    let parts = message.trim().split('|').collect::<Vec<_>>();
    if parts.len() < 4 || parts[0] != "LMC_DEVICE" {
        return None;
    }

    let control_port = parts[3].parse::<u16>().ok()?;
    let ip = from.ip().to_string();

    Some(LanDevice {
        id: format!("{}:{}", ip, control_port),
        name: parts[1].to_string(),
        ip,
        control_port,
        version: parts[2].to_string(),
    })
}

fn ping_device(device: &LanDevice) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| format!("创建心跳通道失败: {err}"))?;
    socket
        .set_read_timeout(Some(Duration::from_millis(250)))
        .map_err(|err| format!("设置心跳超时失败: {err}"))?;

    socket
        .send_to(
            "LMC_PING".as_bytes(),
            format!("{}:{}", device.ip, device.control_port),
        )
        .map_err(|err| format!("发送心跳失败: {err}"))?;

    let begin = Instant::now();
    let mut buffer = [0_u8; 512];

    while begin.elapsed() < Duration::from_millis(PING_TIMEOUT_MS) {
        match socket.recv_from(&mut buffer) {
            Ok((length, from)) => {
                if from.ip().to_string() != device.ip {
                    continue;
                }
                let text = String::from_utf8_lossy(&buffer[..length])
                    .trim()
                    .to_string();
                if text == "LMC_PONG" {
                    return Ok(());
                }
            }
            Err(err)
                if err.kind() == std::io::ErrorKind::WouldBlock
                    || err.kind() == std::io::ErrorKind::TimedOut => {}
            Err(err) => return Err(format!("心跳失败: {err}")),
        }
    }

    Err("手机心跳超时".to_string())
}

fn send_udp_message(ip: &str, port: u16, message: &str) -> Result<(), String> {
    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|err| format!("创建发送通道失败: {err}"))?;
    socket
        .send_to(message.as_bytes(), format!("{}:{}", ip, port))
        .map_err(|err| format!("发送消息失败: {err}"))?;
    Ok(())
}

fn desktop_name() -> String {
    std::env::var("COMPUTERNAME").unwrap_or_else(|_| "LinkMyComputer-PC".to_string())
}

fn start_touch_listener(runtime: Arc<Mutex<TouchRuntime>>) {
    thread::spawn(move || {
        let socket = match UdpSocket::bind(("0.0.0.0", HOST_TOUCH_PORT)) {
            Ok(socket) => socket,
            Err(err) => {
                eprintln!("触控监听启动失败: {err}");
                return;
            }
        };

        if let Err(err) = socket.set_read_timeout(Some(Duration::from_millis(250))) {
            eprintln!("设置触控监听超时失败: {err}");
        }

        let mut buffer = [0_u8; 1024];
        loop {
            match socket.recv_from(&mut buffer) {
                Ok((length, from)) => {
                    let payload = String::from_utf8_lossy(&buffer[..length]).to_string();
                    handle_touch_datagram(&runtime, from, &payload);
                }
                Err(err)
                    if err.kind() == std::io::ErrorKind::WouldBlock
                        || err.kind() == std::io::ErrorKind::TimedOut => {}
                Err(err) => {
                    eprintln!("触控监听异常: {err}");
                }
            }
        }
    });
}

fn handle_touch_datagram(runtime: &Arc<Mutex<TouchRuntime>>, from: SocketAddr, payload: &str) {
    let event = match parse_touch_packet(payload) {
        Some(event) => event,
        None => return,
    };

    let (serial, command) = {
        let mut guard = match runtime.lock() {
            Ok(guard) => guard,
            Err(_) => return,
        };

        let from_ip = from.ip().to_string();
        if guard.connected_device_ip.as_deref() != Some(from_ip.as_str()) {
            return;
        }

        let serial = match ensure_mumu_serial(&mut guard) {
            Ok(serial) => serial,
            Err(_) => return,
        };

        let command = plan_touch_command(&mut guard, &event);
        (serial, command)
    };

    if let Some(command) = command {
        let _ = execute_adb_touch(&serial, command);
    }
}

fn parse_touch_packet(payload: &str) -> Option<TouchEventPacket> {
    let parts = payload.trim().split('|').collect::<Vec<_>>();
    if parts.len() < 8 || parts[0] != "LMC_TOUCH" {
        return None;
    }

    let pointer_id = parts[2].parse::<u8>().ok()?;
    let action = match parts[3] {
        "DOWN" => PointerAction::Down,
        "MOVE" => PointerAction::Move,
        "UP" => PointerAction::Up,
        "CANCEL" => PointerAction::Cancel,
        _ => return None,
    };

    Some(TouchEventPacket {
        pointer_id,
        action,
        x: parts[4].parse::<f32>().ok()?,
        y: parts[5].parse::<f32>().ok()?,
        pressure: parts[6].parse::<f32>().ok()?,
    })
}

fn ensure_mumu_serial(runtime: &mut TouchRuntime) -> Result<String, String> {
    if let Some(serial) = runtime.mumu_serial.clone() {
        return Ok(serial);
    }

    let bridge = MumuBridge::new(runtime.target_width.max(1), runtime.target_height.max(1));
    let serial = bridge
        .discover_serial_via_adb("adb")
        .map_err(|err| format!("无法找到 MuMu 设备: {err}"))?;
    runtime.mumu_serial = Some(serial.clone());
    Ok(serial)
}

fn plan_touch_command(
    runtime: &mut TouchRuntime,
    event: &TouchEventPacket,
) -> Option<AdbTouchCommand> {
    if event.pointer_id != 0 {
        return None;
    }

    let x = to_pixel(event.x, runtime.target_width);
    let y = to_pixel(event.y, runtime.target_height);

    match event.action {
        PointerAction::Down => {
            runtime.down_points.insert(event.pointer_id, (x, y));
            runtime.last_points.insert(event.pointer_id, (x, y));
            None
        }
        PointerAction::Move => {
            runtime.last_points.insert(event.pointer_id, (x, y));
            None
        }
        PointerAction::Up => {
            let start = runtime
                .down_points
                .remove(&event.pointer_id)
                .unwrap_or((x, y));
            let end = runtime
                .last_points
                .remove(&event.pointer_id)
                .unwrap_or((x, y));

            let distance = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
            if distance <= 24 {
                Some(AdbTouchCommand::Tap { x: end.0, y: end.1 })
            } else {
                Some(AdbTouchCommand::Swipe {
                    start_x: start.0,
                    start_y: start.1,
                    end_x: end.0,
                    end_y: end.1,
                    duration_ms: 40,
                })
            }
        }
        PointerAction::Cancel => {
            runtime.down_points.remove(&event.pointer_id);
            runtime.last_points.remove(&event.pointer_id);
            None
        }
    }
}

fn to_pixel(normalized: f32, max: u32) -> u32 {
    let upper = max.max(1) - 1;
    let value = normalized.clamp(0.0, 1.0) * upper as f32;
    value.round() as u32
}

fn execute_adb_touch(serial: &str, command: AdbTouchCommand) -> Result<(), String> {
    let mut process = Command::new("adb");
    process.arg("-s").arg(serial).arg("shell").arg("input");

    match command {
        AdbTouchCommand::Tap { x, y } => {
            process.arg("tap").arg(x.to_string()).arg(y.to_string());
        }
        AdbTouchCommand::Swipe {
            start_x,
            start_y,
            end_x,
            end_y,
            duration_ms,
        } => {
            process
                .arg("swipe")
                .arg(start_x.to_string())
                .arg(start_y.to_string())
                .arg(end_x.to_string())
                .arg(end_y.to_string())
                .arg(duration_ms.to_string());
        }
    }

    let output = process
        .output()
        .map_err(|err| format!("执行 adb 触控失败: {err}"))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stderr.is_empty() {
        return Err("adb 触控命令执行失败（无错误输出）".to_string());
    }
    Err(format!("adb 触控命令执行失败: {stderr}"))
}

fn main() {
    let touch_runtime = Arc::new(Mutex::new(TouchRuntime::default()));
    start_touch_listener(touch_runtime.clone());

    tauri::Builder::default()
        .manage(HostState {
            session: Mutex::new(SessionManager::new()),
            connection: Mutex::new(None),
            touch_runtime,
        })
        .invoke_handler(tauri::generate_handler![
            start_locked_session,
            stop_session,
            session_status,
            scan_lan_devices,
            request_device_connection,
            disconnect_device,
            connection_status
        ])
        .run(tauri::generate_context!())
        .expect("Tauri 应用启动失败");
}
