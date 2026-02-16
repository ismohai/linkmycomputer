# MuMu LAN Remote Control Design

## Goal

Build a low-latency LAN remote control system so a phone can view and control MuMu on Windows with true multi-touch for game play.

## Why This Exists

The user plays mobile games in a desktop emulator and needs phone-native touch control because desktop keyboard and mouse mapping is hard to use. Existing remote tools introduce too much latency.

## Scope

- Phone shows emulator video in real time.
- Phone sends multi-touch gestures to emulator.
- LAN only mode for lowest latency.
- Performance profiles include 60, 90, 120, and 144 FPS.
- Resolution profiles include multiple levels up to 2460x1080.

## Non-Goals

- Internet relay or WAN access.
- iOS client in first release.
- Generic emulator support in first release (MuMu first).

## System Architecture

### Host (Windows)

- Rust `host-core` for capture, encoding, transport, touch injection, and session orchestration.
- Optional Tauri + React UI (`host-ui`) for setup, profile selection, pairing, and diagnostics.

### Mobile (Android)

- Native Kotlin app for video rendering, touch capture, and session control.
- Full multi-pointer tracking (pointer id lifecycle preserved).

### Transport

- WebRTC for video and control channels over LAN.
- Video on media stream.
- Control on DataChannel with low-latency semantics.

## MuMu Input Strategy

- Primary path uses ADB + minitouch pipeline.
- Host discovers MuMu-targetable Android instance and forwards touch commands.
- Touch protocol carries pointer id, action, normalized coordinates, pressure, and timestamp.

## Lock-Performance Policy

- Default mode: `Turbo Lock`.
- Session runs at selected FPS and resolution without adaptive downgrade.
- Pre-flight capability checks decide whether selected profile can start.
- If profile exceeds hardware/network capability, startup fails with clear fallback suggestions.

## Profiles

- FPS: 60 / 90 / 120 / 144.
- Resolution presets include 1280x720, 1600x900, 1920x1080, 2460x1080.
- Codec preference: HEVC first for high profiles; H264 fallback when unsupported.

## Critical Data Flow

1. Host captures MuMu window frames.
2. Host encodes frame by selected profile.
3. Host sends encoded stream via WebRTC.
4. Android decodes and renders immediately.
5. Android touch layer emits normalized multi-touch events.
6. Host receives control packets and injects touches to MuMu via minitouch.

## Error Handling

- Device discovery failure: show explicit remediation steps.
- ADB disconnect: auto-retry and notify in UI.
- Unsupported profile: block start and show closest supported presets.
- Control channel loss: freeze touch input and attempt reconnect.

## Security and LAN Boundaries

- LAN-only discovery with mDNS.
- One-time pairing token via QR.
- Device whitelist after first successful pairing.
- No default external exposure.

## Verification Targets

- Touch correctness: two-finger zoom, three-finger tap, drag, long press.
- Stability: 15-minute continuous session without disconnect.
- Performance: stable frame pacing at selected profile when capability check passes.
- Latency telemetry: capture-to-render and touch-to-inject timings recorded.

## Delivery Phases

- P0: End-to-end PoC with single touch.
- P1: Multi-touch and profile switching.
- P2: 120/144 high profile hardening and diagnostics.
- P3: CI packaging for Windows installer and Android APK.
