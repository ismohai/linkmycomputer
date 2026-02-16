import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import { getSessionStatus, SessionPayload, startLockedSession, stopSession } from "./lib/api";
import { SessionPage, UiSessionStatus } from "./pages/Session";
import "./styles.css";

function HostConsoleApp() {
  const [status, setStatus] = useState<UiSessionStatus>("idle");
  const [lastMessage, setLastMessage] = useState<string>("准备就绪，可随时启动会话。");

  useEffect(() => {
    void refreshStatus();
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
