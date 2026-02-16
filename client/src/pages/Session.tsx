import { FormEvent, useMemo, useState } from "react";

import { SessionPayload } from "../lib/api";

export type UiSessionStatus = "idle" | "starting" | "running" | "error";

type Props = {
  status: UiSessionStatus;
  lastMessage?: string;
  onStartSession: (payload: SessionPayload) => Promise<void> | void;
  onStopSession: () => Promise<void> | void;
};

const FPS_VALUES = [60, 90, 120, 144] as const;
const RESOLUTION_VALUES = ["1280x720", "1600x900", "1920x1080", "2460x1080"] as const;

const STATUS_LABEL: Record<UiSessionStatus, string> = {
  idle: "Idle",
  starting: "Starting",
  running: "Running",
  error: "Error"
};

export function SessionPage({
  status,
  lastMessage,
  onStartSession,
  onStopSession
}: Props) {
  const [fps, setFps] = useState<(typeof FPS_VALUES)[number]>(144);
  const [resolution, setResolution] =
    useState<(typeof RESOLUTION_VALUES)[number]>("2460x1080");
  const [bitrateKbps, setBitrateKbps] = useState<number>(80000);

  const profileSummary = useMemo(() => {
    return `${resolution} @ ${fps}Hz · ${bitrateKbps.toLocaleString()} kbps · turbo_lock`;
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
          <p className="host-eyebrow">LinkMyComputer Host</p>
          <h1>MuMu Turbo Lock Console</h1>
          <p className="host-subtitle">
            Desktop Rust host with fixed high-refresh streaming and low-latency touch bridge.
          </p>
        </header>

        <div className="status-row" role="status" aria-live="polite">
          <span className={`status-dot status-dot--${status}`} aria-hidden="true" />
          <strong>{STATUS_LABEL[status]}</strong>
          <span className="status-message">{lastMessage ?? "Ready to launch session."}</span>
        </div>

        <form onSubmit={onSubmit} className="profile-grid">
          <label htmlFor="fps">FPS</label>
          <select
            id="fps"
            aria-label="FPS"
            value={fps}
            onChange={(event) => setFps(Number(event.target.value) as (typeof FPS_VALUES)[number])}
          >
            {FPS_VALUES.map((value) => (
              <option key={value} value={value}>
                {value}
              </option>
            ))}
          </select>

          <label htmlFor="resolution">Resolution</label>
          <select
            id="resolution"
            aria-label="Resolution"
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

          <label htmlFor="bitrate">Bitrate</label>
          <input
            id="bitrate"
            aria-label="Bitrate"
            type="number"
            min={1000}
            step={1000}
            value={bitrateKbps}
            onChange={(event) => setBitrateKbps(Number(event.target.value))}
          />

          <div className="profile-summary">{profileSummary}</div>

          <div className="actions-row">
            <button type="submit" disabled={status === "starting"}>
              {status === "starting" ? "Starting..." : "Start Locked Session"}
            </button>
            <button
              type="button"
              className="ghost"
              onClick={() => {
                void onStopSession();
              }}
              disabled={status !== "running"}
            >
              Stop Session
            </button>
          </div>
        </form>
      </section>
    </main>
  );
}
