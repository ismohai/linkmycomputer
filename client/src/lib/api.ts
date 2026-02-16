import { invoke } from "@tauri-apps/api/core";

export type LockPolicy = "turbo_lock";

export type SessionPayload = {
  fps: 60 | 90 | 120 | 144;
  resolution: "1280x720" | "1600x900" | "1920x1080" | "2460x1080";
  bitrateKbps: number;
  lockPolicy: LockPolicy;
};

export type SessionState = "idle" | "starting" | "running";

export type SessionStatusResponse = {
  state: SessionState;
};

export type LanDevice = {
  id: string;
  name: string;
  ip: string;
  controlPort: number;
  version: string;
};

export type LanConnectionStatus = {
  connected: boolean;
  device?: LanDevice;
  message: string;
};

export async function startLockedSession(payload: SessionPayload): Promise<SessionPayload> {
  if (hasTauriRuntime()) {
    return invoke<SessionPayload>("start_locked_session", { payload });
  }

  return payload;
}

export async function stopSession(): Promise<void> {
  if (hasTauriRuntime()) {
    await invoke("stop_session");
  }
}

export async function getSessionStatus(): Promise<SessionStatusResponse> {
  if (hasTauriRuntime()) {
    return invoke<SessionStatusResponse>("session_status");
  }

  return { state: "idle" };
}

export async function scanLanDevices(): Promise<LanDevice[]> {
  if (hasTauriRuntime()) {
    return invoke<LanDevice[]>("scan_lan_devices");
  }

  return [];
}

export async function requestDeviceConnection(device: LanDevice): Promise<LanConnectionStatus> {
  if (hasTauriRuntime()) {
    return invoke<LanConnectionStatus>("request_device_connection", { device });
  }

  return {
    connected: false,
    message: "请在桌面应用中运行。"
  };
}

export async function disconnectDevice(): Promise<LanConnectionStatus> {
  if (hasTauriRuntime()) {
    return invoke<LanConnectionStatus>("disconnect_device");
  }

  return {
    connected: false,
    message: "已断开连接。"
  };
}

export async function getConnectionStatus(): Promise<LanConnectionStatus> {
  if (hasTauriRuntime()) {
    return invoke<LanConnectionStatus>("connection_status");
  }

  return {
    connected: false,
    message: "尚未连接手机。"
  };
}

function hasTauriRuntime(): boolean {
  return typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;
}
