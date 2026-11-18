// Tests for api.ts against Tauri's IPC mock (no window, no Rust).
import { clearMocks, mockIPC } from "@tauri-apps/api/mocks";
import { afterEach, describe, expect, it } from "vitest";
import { appVersion } from "./api";

afterEach(() => clearMocks());

describe("appVersion", () => {
  it("invokes the app_version command and returns its result", async () => {
    const calls: string[] = [];
    mockIPC((cmd) => {
      calls.push(cmd);
      return "1.2.3";
    });
    await expect(appVersion()).resolves.toBe("1.2.3");
    expect(calls).toEqual(["app_version"]);
  });

  it("passes errors from the command through", async () => {
    mockIPC(() => {
      throw new Error("boom");
    });
    await expect(appVersion()).rejects.toThrow("boom");
  });
});
