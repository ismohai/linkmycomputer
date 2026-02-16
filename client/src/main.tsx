import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import {
  disconnectDevice,
  getConnectionStatus,
  getSessionStatus,
  LanConnectionStatus,
  LanDevice,
  requestDeviceConnection,
  scanLanDevices,
  SessionPayload,
  startLockedSession,
  stopSession
} from "./lib/api";
import { SessionPage, UiSessionStatus } from "./pages/Session";
import "./styles.css";

function HostConsoleApp() {
  const [status, setStatus] = useState<UiSessionStatus>("idle");
  const [lastMessage, setLastMessage] = useState<string>("准备就绪，可随时启动会话。");
  const [devices, setDevices] = useState<LanDevice[]>([]);
  const [scanning, setScanning] = useState(false);
  const [connection, setConnection] = useState<LanConnectionStatus>({
    connected: false,
    message: "尚未连接手机。"
  });

  useEffect(() => {
    void refreshStatus();
    void refreshConnectionStatus();

    const timer = window.setInterval(() => {
      void refreshConnectionStatus();
    }, 3000);

    return () => {
      window.clearInterval(timer);
    };
  }, []);

  const refreshStatus = async () => {
    try {
      const response = await getSessionStatus();
      setStatus(mapBackendState(response.state));
    } catch (error) {
      setStatus("error");
      setLastMessage(formatError(error));
    }
  };

  const refreshConnectionStatus = async () => {
    try {
      const response = await getConnectionStatus();
      setConnection(response);
    } catch (error) {
      setConnection({
        connected: false,
        message: formatError(error)
      });
    }
  };

  const handleScanDevices = async () => {
    try {
      setScanning(true);
      const response = await scanLanDevices();
      setDevices(response);
      if (response.length === 0) {
        setLastMessage("扫描完成：未发现可连接手机。请确认手机 App 已打开。");
      } else {
        setLastMessage(`扫描完成：发现 ${response.length} 台手机设备。`);
      }
    } catch (error) {
      setLastMessage(`扫描失败：${formatError(error)}`);
    } finally {
      setScanning(false);
    }
  };

  const handleConnectDevice = async (device: LanDevice) => {
    try {
      setLastMessage(`正在向 ${device.name} 发起连接请求...`);
      const response = await requestDeviceConnection(device);
      setConnection(response);
      setLastMessage(response.message);
    } catch (error) {
      const message = formatError(error);
      setConnection({ connected: false, message });
      setLastMessage(`连接失败：${message}`);
    }
  };

  const handleDisconnectDevice = async () => {
    try {
      const response = await disconnectDevice();
      setConnection(response);
      setLastMessage(response.message);
    } catch (error) {
      const message = formatError(error);
      setLastMessage(`断开失败：${message}`);
    }
  };

  const handleStart = async (payload: SessionPayload) => {
    try {
      setStatus("starting");
      setLastMessage(`正在启动 ${payload.resolution}@${payload.fps}Hz 配置...`);
      await startLockedSession(payload);
      setStatus("running");
      setLastMessage(`已运行：${payload.resolution}@${payload.fps}Hz（极速锁定）`);
    } catch (error) {
      setStatus("error");
      setLastMessage(formatError(error));
    }
  };

  const handleStop = async () => {
    try {
      await stopSession();
      setStatus("idle");
      setLastMessage("会话已停止。");
    } catch (error) {
      setStatus("error");
      setLastMessage(formatError(error));
    }
  };

  return (
    <SessionPage
      status={status}
      lastMessage={lastMessage}
      devices={devices}
      scanning={scanning}
      connection={connection}
      onScanDevices={handleScanDevices}
      onConnectDevice={handleConnectDevice}
      onDisconnectDevice={handleDisconnectDevice}
      onStartSession={handleStart}
      onStopSession={handleStop}
    />
  );
}

function mapBackendState(value: "idle" | "starting" | "running"): UiSessionStatus {
  return value;
}

function formatError(error: unknown): string {
  if (typeof error === "string") {
    return error;
  }
  if (error && typeof error === "object" && "message" in error) {
    return String((error as { message: string }).message);
  }
  return "控制会话时发生未知错误。";
}

const root = document.getElementById("root");

if (root) {
  createRoot(root).render(
    <React.StrictMode>
      <HostConsoleApp />
    </React.StrictMode>
  );
}
