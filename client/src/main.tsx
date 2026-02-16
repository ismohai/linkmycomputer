import React, { useEffect, useState } from "react";
import { createRoot } from "react-dom/client";

import { getSessionStatus, SessionPayload, startLockedSession, stopSession } from "./lib/api";
import { SessionPage, UiSessionStatus } from "./pages/Session";
import "./styles.css";

function HostConsoleApp() {
  const [status, setStatus] = useState<UiSessionStatus>("idle");
  const [lastMessage, setLastMessage] = useState<string>("Ready to launch session.");

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
      setLastMessage(`Starting ${payload.resolution}@${payload.fps}Hz profile...`);
      await startLockedSession(payload);
      setStatus("running");
      setLastMessage(`Running ${payload.resolution}@${payload.fps}Hz in Turbo Lock.`);
    } catch (error) {
      setStatus("error");
      setLastMessage(formatError(error));
    }
  };

  const handleStop = async () => {
    try {
      await stopSession();
      setStatus("idle");
      setLastMessage("Session stopped.");
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
  return "Unexpected error while controlling session.";
}

const root = document.getElementById("root");

if (root) {
  createRoot(root).render(
    <React.StrictMode>
      <HostConsoleApp />
    </React.StrictMode>
  );
}
