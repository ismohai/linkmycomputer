use host_core::input::mumu::adb::{find_mumu_candidate, parse_adb_devices, AdbDeviceState};
use host_core::input::mumu::bridge::MumuBridge;
use host_core::input::mumu::minitouch::{MinitouchBuilder, TouchPoint};
use host_core::protocol::control::{PointerAction, PointerEvent};

#[test]
fn parse_adb_devices_extracts_online_rows() {
    let sample = "List of devices attached\n127.0.0.1:7555\tdevice\nemulator-5554\toffline\n";
    let devices = parse_adb_devices(sample);

    assert_eq!(devices.len(), 2);
    assert_eq!(devices[0].serial, "127.0.0.1:7555");
    assert_eq!(devices[0].state, AdbDeviceState::Device);
    assert_eq!(devices[1].state, AdbDeviceState::Offline);
}

#[test]
fn pick_mumu_candidate_prefers_local_loopback_serial() {
    let sample = "List of devices attached\n192.168.1.11:7555\tdevice\n127.0.0.1:7555\tdevice\n";
    let devices = parse_adb_devices(sample);

    let picked = find_mumu_candidate(&devices).expect("candidate");
    assert_eq!(picked.serial, "127.0.0.1:7555");
}

#[test]
fn minitouch_builder_encodes_multitouch_frame() {
    let payload = MinitouchBuilder::default()
        .down(TouchPoint::new(0, 120, 300, 50))
        .move_to(TouchPoint::new(0, 150, 350, 40))
        .up(0)
        .commit()
        .to_string();

    assert_eq!(payload, "d 0 120 300 50\nm 0 150 350 40\nu 0\nc\n");
}

#[test]
fn bridge_picks_target_serial_from_adb_output() {
    let bridge = MumuBridge::new(2460, 1080);
    let sample = "List of devices attached\n192.168.1.20:7555\tdevice\n127.0.0.1:7555\tdevice\n";

    let serial = bridge
        .discover_serial_from_adb_output(sample)
        .expect("serial should be discovered");

    assert_eq!(serial, "127.0.0.1:7555");
}

#[test]
fn bridge_translates_touch_events_into_minitouch_payload() {
    let bridge = MumuBridge::new(2460, 1080);
    let payload = bridge
        .build_minitouch_payload(&[
            PointerEvent {
                pointer_id: 0,
                action: PointerAction::Down,
                x: 0.5,
                y: 0.5,
                pressure: 0.8,
                timestamp_ms: 1,
            },
            PointerEvent {
                pointer_id: 0,
                action: PointerAction::Up,
                x: 0.5,
                y: 0.5,
                pressure: 0.2,
                timestamp_ms: 2,
            },
        ])
        .expect("payload should build");

    assert_eq!(payload, "d 0 1230 540 80\nu 0\nc\n");
}
