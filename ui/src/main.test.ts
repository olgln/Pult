// Tests for the entry point: it mounts the app into #app and fails loudly without it.
import { afterEach, describe, expect, it, vi } from "vitest";

vi.mock("./lib/api", () => ({ appVersion: () => Promise.resolve("0.1.0") }));

afterEach(() => {
  document.body.innerHTML = "";
  vi.resetModules();
});

describe("main", () => {
  it("mounts the app into #app", async () => {
    document.body.innerHTML = '<div id="app"></div>';
    await import("./main");
    expect(document.querySelector("#app aside")).not.toBeNull();
  });

  it("throws when #app is missing", async () => {
    await expect(import("./main")).rejects.toThrow("#app element is missing");
  });
});
