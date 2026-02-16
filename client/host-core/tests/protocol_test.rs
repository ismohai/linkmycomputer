use host_core::protocol::control::{ControlFrame, PointerAction, PointerEvent, TouchEnvelope};

#[test]
fn touch_envelope_roundtrip_keeps_pointer_lifecycle() {
    let down = PointerEvent {
        pointer_id: 7,
        action: PointerAction::Down,
        x: 0.42,
        y: 0.88,
        pressure: 0.7,
        timestamp_ms: 101,
    };

    let mv = PointerEvent {
        pointer_id: 7,
        action: PointerAction::Move,
        x: 0.50,
        y: 0.70,
        pressure: 0.6,
        timestamp_ms: 121,
    };

    let up = PointerEvent {
        pointer_id: 7,
        action: PointerAction::Up,
        x: 0.51,
        y: 0.66,
        pressure: 0.2,
        timestamp_ms: 150,
    };

    let frame = ControlFrame::Touch(TouchEnvelope {
        frame_id: 9,
        events: vec![down, mv, up],
    });

    let bytes = frame.to_wire_bytes().expect("serialize");
    let decoded = ControlFrame::from_wire_bytes(&bytes).expect("deserialize");

    assert_eq!(frame, decoded);
}

#[test]
fn decode_rejects_empty_payload() {
    let err = ControlFrame::from_wire_bytes(&[]).expect_err("empty payload must fail");
    assert!(err.to_string().contains("payload is empty"));
}

#[test]
fn decode_rejects_out_of_range_touch_coordinates() {
    let invalid = ControlFrame::Touch(TouchEnvelope {
        frame_id: 1,
        events: vec![PointerEvent {
            pointer_id: 0,
            action: PointerAction::Move,
            x: 1.2,
            y: 0.5,
            pressure: 0.6,
            timestamp_ms: 11,
        }],
    });

    let payload = invalid.to_wire_bytes().expect("serialize");
    let err = ControlFrame::from_wire_bytes(&payload)
        .expect_err("invalid normalized coordinate must fail");
    assert!(err
        .to_string()
        .contains("touch event x/y must be finite and within [0.0, 1.0]"));
}
