# MuMu 局域网远控实施计划

> 执行要求：按任务拆分逐项实现、逐项验证。

**目标：** 交付可打包的 MuMu 局域网远控工程，覆盖桌面端控制台、主机核心、安卓端基础能力与 GitHub 自动打包。  
**架构：** `client/host-core` 负责核心链路，`client` 负责 Tauri 桌面 UI，`app` 负责 Android 端触控与后续渲染能力。  
**技术栈：** Rust、Tauri、React、Kotlin、Gradle、GitHub Actions。

---

## 任务 1：工程骨架与目录规范

**文件：**
- 新建/调整：`client/`、`client/host-core/`、`app/`
- 修改：`Cargo.toml`、`README.md`

**步骤：**
1. 明确目录规范并建立基础结构。  
2. 运行 `cargo test -p host-core` 验证主机核心可编译。  
3. 运行 `npm test`（在 `client`）验证 UI 工程可运行。

## 任务 2：主机核心锁档配置

**文件：**
- `client/host-core/src/config/profile.rs`
- `client/host-core/tests/profile_test.rs`

**步骤：**
1. 先写失败测试（非法帧率、非法分辨率、无效码率）。  
2. 实现最小配置模型并校验通过。  
3. 运行 `cargo test -p host-core --test profile_test`。

## 任务 3：控制协议与校验

**文件：**
- `shared/proto/control.proto`
- `client/host-core/src/protocol/control.rs`
- `client/host-core/tests/protocol_test.rs`

**步骤：**
1. 先写失败测试（空负载、越界坐标、序列化回环）。  
2. 完成协议模型、编解码与边界校验。  
3. 运行 `cargo test -p host-core --test protocol_test`。

## 任务 4：MuMu 触控桥接

**文件：**
- `client/host-core/src/input/mumu/adb.rs`
- `client/host-core/src/input/mumu/minitouch.rs`
- `client/host-core/src/input/mumu/bridge.rs`
- `client/host-core/tests/mumu_test.rs`

**步骤：**
1. 先写失败测试（ADB 解析、候选设备选择、payload 编码）。  
2. 完成 MuMu 设备发现与 minitouch 指令转换。  
3. 运行 `cargo test -p host-core --test mumu_test`。

## 任务 5：映射与会话编排

**文件：**
- `client/host-core/src/input/mapping.rs`
- `client/host-core/src/session.rs`
- `client/host-core/tests/mapping_test.rs`
- `client/host-core/tests/session_test.rs`

**步骤：**
1. 先写失败测试（黑边区域、状态切换、失败回滚）。  
2. 实现映射逻辑与会话状态机。  
3. 运行 `cargo test -p host-core --test mapping_test` 与 `cargo test -p host-core --test session_test`。

## 任务 6：桌面端 Tauri 控制台

**文件：**
- `client/src/pages/Session.tsx`
- `client/src/main.tsx`
- `client/src/lib/api.ts`
- `client/src-tauri/src/main.rs`
- `client/src/pages/Session.test.tsx`

**步骤：**
1. 先写失败测试（参数提交、停止按钮状态）。  
2. 实现启动/停止/状态查询三条命令链路。  
3. 运行 `npm test`、`npm run build`、`cargo check`（`client/src-tauri`）。

## 任务 7：安卓端基础工程

**文件：**
- `app/app/src/main/java/com/linkmycomputer/player/TouchTracker.kt`
- `app/app/src/main/java/com/linkmycomputer/player/TouchOverlayView.kt`
- `app/app/src/main/java/com/linkmycomputer/player/PlayerActivity.kt`
- `app/app/src/test/java/com/linkmycomputer/player/TouchTrackerTest.kt`

**步骤：**
1. 先写失败测试（pointer 生命周期与范围裁剪）。  
2. 实现触控跟踪和视图事件分发。  
3. 运行 `gradle -p app test`。

## 任务 8：GitHub 自动打包

**文件：**
- `.github/workflows/release-host.yml`
- `.github/workflows/release-android.yml`

**步骤：**
1. 桌面端工作流覆盖：Rust 测试、客户端测试、构建、产物上传。  
2. 安卓工作流覆盖：单元测试、APK 构建、产物上传。  
3. 手动触发工作流并确认产物可下载。

## 任务 9：最终验收

**步骤：**
1. `cargo test -p host-core`  
2. `cargo fmt --all -- --check`  
3. `npm test`（`client`）  
4. `npm run build`（`client`）  
5. `cargo check`（`client/src-tauri`）

通过后发布到 GitHub 主分支并触发打包工作流。
