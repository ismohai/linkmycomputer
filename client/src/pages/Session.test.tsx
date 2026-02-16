import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { SessionPage } from "./Session";

describe("SessionPage", () => {
  it("serializes Turbo Lock profile payload", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();
    render(
      <SessionPage
        status="idle"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    fireEvent.change(screen.getByLabelText(/fps/i), { target: { value: "144" } });
    fireEvent.change(screen.getByLabelText(/resolution/i), {
      target: { value: "2460x1080" }
    });
    fireEvent.change(screen.getByLabelText(/bitrate/i), { target: { value: "80000" } });
    fireEvent.click(screen.getByRole("button", { name: /start locked session/i }));

    expect(onStart).toHaveBeenCalledWith({
      fps: 144,
      resolution: "2460x1080",
      bitrateKbps: 80000,
      lockPolicy: "turbo_lock"
    });
  });

  it("disables stop button when not running", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    const { rerender } = render(
      <SessionPage
        status="idle"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /stop session/i })).toBeDisabled();

    rerender(
      <SessionPage
        status="running"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /stop session/i })).toBeEnabled();
  });
});
