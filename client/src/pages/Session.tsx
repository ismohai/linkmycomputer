import { FormEvent, useMemo, useState } from "react";

import { LanDevice, SessionPayload } from "../lib/api";

export type UiSessionStatus = "idle" | "starting" | "running" | "error";

type Props = {
  status: UiSessionStatus;
  lastMessage?: string;
  devices: LanDevice[];
  scanning: boolean;
  connection: {
    connected: boolean;
    message: string;
    device?: LanDevice;
  };
  onScanDevices: () => Promise<void> | void;
  onConnectDevice: (device: LanDevice) => Promise<void> | void;
  onDisconnectDevice: () => Promise<void> | void;
  onStartSession: (payload: SessionPayload) => Promise<void> | void;
  onStopSession: () => Promise<void> | void;
};

const FPS_VALUES = [60, 90, 120, 144] as const;
const RESOLUTION_VALUES = ["1280x720", "1600x900", "1920x1080", "2460x1080"] as const;

const STATUS_LABEL: Record<UiSessionStatus, string> = {
  idle: "空闲",
  starting: "启动中",
  running: "运行中",
  error: "错误"
};

export function SessionPage({
  status,
  lastMessage,
  devices,
  scanning,
  connection,
  onScanDevices,
  onConnectDevice,
  onDisconnectDevice,
  onStartSession,
  onStopSession
}: Props) {
  const [fps, setFps] = useState<(typeof FPS_VALUES)[number]>(144);
  const [resolution, setResolution] =
    useState<(typeof RESOLUTION_VALUES)[number]>("2460x1080");
  const [bitrateKbps, setBitrateKbps] = useState<number>(80000);

  const profileSummary = useMemo(() => {
    return `${resolution} @ ${fps}Hz · ${bitrateKbps.toLocaleString()} kbps · 极速锁定`;
  }, [bitrateKbps, fps, resolution]);

  const onSubmit = async (event: FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    await onStartSession({
      fps,
      resolution,
      bitrateKbps,
      lockPolicy: "turbo_lock"
    });
  };

  return (
    <main className="host-shell">
      <section className="host-panel">
        <header className="host-panel__header">
          <p className="host-eyebrow">LinkMyComputer 桌面端</p>
          <h1>MuMu 极速锁定控制台</h1>
          <p className="host-subtitle">
            Rust 主机核心 + 低延迟触控桥接，支持局域网固定高刷配置。
          </p>
        </header>

        <div className="status-row" role="status" aria-live="polite">
          <span className={`status-dot status-dot--${status}`} aria-hidden="true" />
          <strong>{STATUS_LABEL[status]}</strong>
          <span className="status-message">{lastMessage ?? "准备就绪，可随时启动会话。"}</span>
        </div>

        <section className="connect-panel">
          <div className="connect-panel__header">
            <h2>手机连接</h2>
            <button
              type="button"
              className="ghost"
              onClick={() => {
                void onScanDevices();
              }}
              disabled={scanning}
            >
              {scanning ? "扫描中..." : "扫描局域网设备"}
            </button>
          </div>

          <p className="connect-hint">{connection.message}</p>

          {connection.connected && connection.device ? (
            <div className="connected-card">
              <strong>已连接：{connection.device.name}</strong>
              <span>
                {connection.device.ip}:{connection.device.controlPort}
              </span>
              <button
                type="button"
                className="ghost"
                onClick={() => {
                  void onDisconnectDevice();
                }}
              >
                断开手机连接
              </button>
            </div>
          ) : null}

          <ul className="device-list">
            {devices.map((device) => (
              <li key={device.id} className="device-item">
                <div>
                  <strong>{device.name}</strong>
                  <p>
                    {device.ip}:{device.controlPort} · 协议 {device.version}
                  </p>
                </div>
                <button
                  type="button"
                  onClick={() => {
                    void onConnectDevice(device);
                  }}
                  disabled={connection.connected}
                >
                  发起连接
                </button>
              </li>
            ))}
          </ul>

          {devices.length === 0 ? (
            <p className="empty-device">未发现手机。请确保手机与电脑在同一局域网，且手机 App 已打开。</p>
          ) : null}
        </section>

        <form onSubmit={onSubmit} className="profile-grid">
          <label htmlFor="fps">帧率 (FPS)</label>
          <select
            id="fps"
            aria-label="帧率"
            value={fps}
            onChange={(event) => setFps(Number(event.target.value) as (typeof FPS_VALUES)[number])}
          >
            {FPS_VALUES.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>

          <label htmlFor="resolution">分辨率</label>
          <select
            id="resolution"
            aria-label="分辨率"
            value={resolution}
            onChange={(event) =>
              setResolution(event.target.value as (typeof RESOLUTION_VALUES)[number])
            }
          >
            {RESOLUTION_VALUES.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>

          <label htmlFor="bitrate">码率 (kbps)</label>
          <input
            id="bitrate"
            aria-label="码率"
            type="number"
            min={1000}
            step={1000}
            value={bitrateKbps}
            onChange={(event) => setBitrateKbps(Number(event.target.value))}
          />

          <div className="profile-summary">{profileSummary}</div>

          <div className="actions-row">
            <button type="submit" disabled={status === "starting"}>
              {status === "starting" ? "启动中..." : "启动锁定会话"}
            </button>
            <button
              type="button"
              className="ghost"
              onClick={() => {
                void onStopSession();
              }}
              disabled={status !== "running"}
            >
              停止会话
            </button>
          </div>
        </form>
      </section>
    </main>
  );
}
