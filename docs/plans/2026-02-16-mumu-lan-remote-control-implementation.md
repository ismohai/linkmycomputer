# MuMu LAN Remote Control Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Deliver a production-ready LAN remote control system for MuMu with phone video + multi-touch control and lockable high refresh/resolution profiles.

**Architecture:** Build a Rust host core for capture/encoding/transport/input injection and a Kotlin Android client for render/touch capture. Use WebRTC media/data channels and a MuMu-focused ADB + minitouch touch pipeline. Keep profile behavior locked by default with pre-flight checks.

**Tech Stack:** Rust, Tokio, Serde, thiserror, Tauri + React, Kotlin, Android WebRTC, ADB/minitouch, GitHub Actions.

---

### Task 1: Workspace Scaffold and Config Profiles

**Files:**
- Create: `Cargo.toml`
- Create: `apps/host-core/Cargo.toml`
- Create: `apps/host-core/src/config/profile.rs`
- Create: `apps/host-core/tests/profile_test.rs`

**Step 1: Write the failing test**

Add tests that fail for missing lock profile validation and fallback listing.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core profile`  
Expected: fail because profile module is missing.

**Step 3: Write minimal implementation**

Implement profile enums, lock policy, and validation helpers.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core profile`  
Expected: all profile tests pass.

### Task 2: Control Protocol Model

**Files:**
- Create: `shared/proto/control.proto`
- Create: `apps/host-core/src/protocol/control.rs`
- Create: `apps/host-core/tests/protocol_test.rs`

**Step 1: Write the failing test**

Add serialization tests for pointer lifecycle and binary payload integrity.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core protocol`  
Expected: fail because protocol model does not exist.

**Step 3: Write minimal implementation**

Implement touch event model and serde codec.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core protocol`  
Expected: tests pass.

### Task 3: MuMu Discovery and minitouch Encoder

**Files:**
- Create: `apps/host-core/src/input/mumu/adb.rs`
- Create: `apps/host-core/src/input/mumu/minitouch.rs`
- Create: `apps/host-core/tests/mumu_test.rs`

**Step 1: Write the failing test**

Add tests for ADB output parsing and minitouch command frame encoding.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core mumu`  
Expected: fail due missing discovery/encoder.

**Step 3: Write minimal implementation**

Implement deterministic parser and command builder for down/move/up/commit.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core mumu`  
Expected: tests pass.

### Task 4: Coordinate Mapping Core

**Files:**
- Create: `apps/host-core/src/input/mapping.rs`
- Create: `apps/host-core/tests/mapping_test.rs`

**Step 1: Write the failing test**

Add tests for letterbox clipping, scaling, and normalized-to-emulator transforms.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core mapping`  
Expected: fail due missing mapping module.

**Step 3: Write minimal implementation**

Implement viewport model and transform math.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core mapping`  
Expected: tests pass.

### Task 5: Capture, Encode, and Transport Abstractions

**Files:**
- Create: `apps/host-core/src/capture/dxgi.rs`
- Create: `apps/host-core/src/encode/nvenc.rs`
- Create: `apps/host-core/src/transport/webrtc.rs`
- Create: `apps/host-core/tests/pipeline_test.rs`

**Step 1: Write the failing test**

Add tests for capability checks and lock-profile pipeline selection.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core pipeline`  
Expected: fail because pipeline modules do not exist.

**Step 3: Write minimal implementation**

Implement capability negotiation, profile gating, and pipeline descriptors.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core pipeline`  
Expected: tests pass.

### Task 6: Android Client Skeleton

**Files:**
- Create: `apps/android/app/src/main/java/com/linkmycomputer/player/PlayerActivity.kt`
- Create: `apps/android/app/src/main/java/com/linkmycomputer/player/TouchOverlayView.kt`
- Create: `apps/android/app/src/main/java/com/linkmycomputer/player/TouchTracker.kt`
- Create: `apps/android/app/src/test/java/com/linkmycomputer/player/TouchTrackerTest.kt`

**Step 1: Write the failing test**

Add tests for pointer lifecycle and stable touch frame generation.

**Step 2: Run test to verify it fails**

Run: `./gradlew test`  
Expected: fail before tracker implementation.

**Step 3: Write minimal implementation**

Implement touch tracker and UI overlay wiring.

**Step 4: Run test to verify it passes**

Run: `./gradlew test`  
Expected: touch tracker tests pass.

### Task 7: Host UI Skeleton

**Files:**
- Create: `apps/host-ui/src/pages/Session.tsx`
- Create: `apps/host-ui/src/lib/api.ts`
- Create: `apps/host-ui/src-tauri/src/main.rs`

**Step 1: Write the failing test**

Add test for profile form state and lock policy serialization.

**Step 2: Run test to verify it fails**

Run: `npm test --workspace apps/host-ui`  
Expected: fail due missing component.

**Step 3: Write minimal implementation**

Implement profile panel and host command bridge.

**Step 4: Run test to verify it passes**

Run: `npm test --workspace apps/host-ui`  
Expected: tests pass.

### Task 8: CI/CD Workflows

**Files:**
- Create: `.github/workflows/release-host.yml`
- Create: `.github/workflows/release-android.yml`

**Step 1: Write the failing test**

Add validation by running local workflow lint or dry run command.

**Step 2: Run validation to verify it fails**

Run: `actionlint`  
Expected: fail before workflow files exist.

**Step 3: Write minimal implementation**

Implement release workflows for Windows host and Android APK.

**Step 4: Run validation to verify it passes**

Run: `actionlint`  
Expected: no workflow lint errors.

### Task 9: End-to-End Session Orchestration

**Files:**
- Create: `apps/host-core/src/session.rs`
- Create: `apps/host-core/tests/session_test.rs`

**Step 1: Write the failing test**

Add tests for startup gating, profile lock enforcement, and session state transitions.

**Step 2: Run test to verify it fails**

Run: `cargo test -p host-core session`  
Expected: fail because session orchestrator is missing.

**Step 3: Write minimal implementation**

Implement session orchestrator and status machine.

**Step 4: Run test to verify it passes**

Run: `cargo test -p host-core session`  
Expected: tests pass.

### Task 10: Full Verification and Packaging Readiness

**Files:**
- Modify: `README.md`
- Modify: `docs/plans/2026-02-16-mumu-lan-remote-control-design.md`

**Step 1: Run verification suite**

Run: `cargo test -p host-core`  
Run: `cargo fmt --all -- --check`

**Step 2: Run client verification**

Run: `./gradlew test`  
Run: `npm test --workspace apps/host-ui`

**Step 3: Document release commands**

Add concrete build and release instructions for host and Android artifacts.

**Step 4: Final status check**

Ensure all tests pass and instructions are reproducible.
