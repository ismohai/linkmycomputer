import { fireEvent, render, screen } from "@testing-library/react";
import { describe, expect, it, vi } from "vitest";

import { SessionPage } from "./Session";

const baseProps = {
  devices: [],
  scanning: false,
  connection: {
    connected: false,
    message: "尚未连接手机。"
  },
  onScanDevices: vi.fn(),
  onConnectDevice: vi.fn(),
  onDisconnectDevice: vi.fn()
};

describe("SessionPage", () => {
  it("可以正确提交极速锁定配置", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();
    render(
      <SessionPage
        status="idle"
        {...baseProps}
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
        {...baseProps}
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /停止会话/i })).toBeDisabled();

    rerender(
      <SessionPage
        status="running"
        {...baseProps}
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByRole("button", { name: /停止会话/i })).toBeEnabled();
  });

  it("可以显示扫描到的手机并发起连接", () => {
    const onStart = vi.fn();
    const onStop = vi.fn();
    const onConnect = vi.fn();

    render(
      <SessionPage
        status="idle"
        {...baseProps}
        devices={[
          {
            id: "192.168.1.21:42043",
            name: "Pixel 8",
            ip: "192.168.1.21",
            controlPort: 42043,
            version: "0.1.0"
          }
        ]}
        onConnectDevice={onConnect}
        onStartSession={onStart}
        onStopSession={onStop}
      />
    );

    expect(screen.getByText(/Pixel 8/i)).toBeInTheDocument();
    fireEvent.click(screen.getByRole("button", { name: /发起连接/i }));
    expect(onConnect).toHaveBeenCalled();
  });
});
