import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { SessionPage } from "./Session";

describe("SessionPage", () => {
  it("可以正确提交极速锁定配置", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();
    render(
      <SessionPage
        status="idle"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    fireEvent.change(screen.getByLabelText(/帧率/i), { target: { value: "144" } });
    fireEvent.change(screen.getByLabelText(/分辨率/i), {
      target: { value: "2460x1080" }
    });
    fireEvent.change(screen.getByLabelText(/码率/i), { target: { value: "80000" } });
    fireEvent.click(screen.getByRole("button", { name: /启动锁定会话/i }));

    expect(onStart).toHaveBeenCalledWith({
      fps: 144,
      resolution: "2460x1080",
      bitrateKbps: 80000,
      lockPolicy: "turbo_lock"
    });
  });

  it("仅在运行中允许停止会话", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();

    const { rerender } = render(
      <SessionPage
        status="idle"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /停止会话/i })).toBeDisabled();

    rerender(
      <SessionPage
        status="running"
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /停止会话/i })).toBeEnabled();
  });
});
