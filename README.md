# LinkMyComputer

面向 MuMu 模拟器的低延迟局域网远程控制项目。

## 项目结构

- `client/host-core`：Rust 核心，负责配置锁定、协议处理、MuMu 触控桥接、会话状态管理。
- `client`：Tauri + React 桌面端控制台（可直接启动 UI）。
- `app`：Android 原生客户端工程（触控采集与后续视频播放接入）。
- `shared/proto`：跨端协议定义。

## 性能档位

- 帧率预设：`60 / 90 / 120 / 144`
- 分辨率预设：`1280x720`、`1600x900`、`1920x1080`、`2460x1080`
- 默认策略：`Turbo Lock`（会话期间固定档位）

## 本地开发

### 环境要求

- Rust stable
- Node.js 20+
- JDK 17+ 与 Gradle（用于 Android 本地验证）

### 核心服务（Rust）

```bash
cargo test -p host-core
cargo run -p host-core -- --fps 144 --resolution 2460x1080 --bitrate 80000 --codec hevc
```

### Android 单元测试

```bash
cd app
gradle test
```

### 桌面端 UI 调试

```bash
cd client
npm test
npm run build
npm run tauri dev
```

## GitHub 打包

- 所有打包流程由 GitHub Actions 执行，工作流位于 `.github/workflows`。
- 桌面端打包：`.github/workflows/release-host.yml`
- 安卓端打包：`.github/workflows/release-android.yml`
