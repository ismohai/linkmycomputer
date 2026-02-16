# LinkMyComputer

Low-latency LAN remote control system for MuMu emulator.

## Architecture

- `client/host-core`: Rust core for profile lock, transport protocol, MuMu touch bridge, and session logic.
- `client`: Tauri + React desktop control panel.
- `app`: Native Android app for video playback and multi-touch capture.
- `shared/proto`: cross-platform protocol schema.

## Performance Profiles

- FPS presets: 60 / 90 / 120 / 144
- Resolution presets: 1280x720, 1600x900, 1920x1080, 2460x1080
- Default lock policy: Turbo Lock (fixed profile during session)

## Local Development

### Prerequisites

- Rust stable toolchain
- Node.js 20+
- JDK 17+ and Gradle (for Android local verification)

### Rust core

```bash
cargo test -p host-core
cargo run -p host-core -- --fps 144 --resolution 2460x1080 --bitrate 80000 --codec hevc
```

### Android unit tests

```bash
cd app
gradle test
```

### Host UI tests

```bash
cd client
npm test
npm run build
npm run tauri dev
```

## Release

- Windows host package and Android APK are built by GitHub Actions workflows in `.github/workflows`.
- Host workflow: `.github/workflows/release-host.yml`
- Android workflow: `.github/workflows/release-android.yml`
