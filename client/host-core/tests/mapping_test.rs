use host_core::input::mapping::ViewportMapping;

#[test]
fn normalized_to_emulator_maps_center_point() {
    let mapping = ViewportMapping::for_letterboxed(2400, 1200, 1920, 1080).expect("valid mapping");

    let point = mapping
        .normalized_to_emulator(0.5, 0.5)
        .expect("center should map");

    assert_eq!(point.x, 960);
    assert_eq!(point.y, 540);
}

#[test]
fn window_to_emulator_rejects_black_bar_input() {
    let mapping = ViewportMapping::for_letterboxed(2400, 1200, 1920, 1080).expect("valid mapping");

    let err = mapping
        .window_to_emulator(40, 600)
        .expect_err("black bar coordinate must fail");

    assert!(err.to_string().contains("outside active emulator viewport"));
}

#[test]
fn normalized_input_must_be_in_range() {
    let mapping = ViewportMapping::for_letterboxed(2400, 1200, 1920, 1080).expect("valid mapping");

    let err = mapping
        .normalized_to_emulator(1.1, 0.5)
        .expect_err("out of range normalized coordinate must fail");

    assert!(err
        .to_string()
        .contains("normalized coordinates must be in range [0.0, 1.0]"));
}
