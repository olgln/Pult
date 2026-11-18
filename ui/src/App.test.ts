// Tests for the window shell with the api module mocked.
import { render, screen, waitFor } from "@testing-library/svelte";
import { beforeEach, describe, expect, it, vi } from "vitest";
import App from "./App.svelte";
import { appVersion } from "./lib/api";

vi.mock("./lib/api", () => ({ appVersion: vi.fn() }));
const mocked = vi.mocked(appVersion);

// Block body on purpose: a function returned from beforeEach is run as a teardown hook.
beforeEach(() => {
  mocked.mockReset();
});

describe("App", () => {
  it("renders both columns and a drag strip, with a loading version", async () => {
    const pending = Promise.withResolvers<string>();
    mocked.mockReturnValue(pending.promise);
    const { container } = render(App);
    expect(screen.getByRole("complementary", { name: "Sessions" })).toBeTruthy();
    expect(screen.getByRole("main")).toBeTruthy();
    expect(container.querySelector("[data-tauri-drag-region]")).not.toBeNull();
    expect(screen.getByTestId("version").textContent).toBe("…");
    pending.resolve("0.1.0");
    await waitFor(() => expect(screen.getByTestId("version").textContent).toBe("v0.1.0"));
  });

  it("shows the version reported by the app", async () => {
    mocked.mockResolvedValue("0.1.0");
    render(App);
    await waitFor(() => expect(screen.getByTestId("version").textContent).toBe("v0.1.0"));
    expect(mocked).toHaveBeenCalledOnce();
  });

  it("says so when the version cannot be loaded", async () => {
    mocked.mockRejectedValue(new Error("no backend"));
    render(App);
    await waitFor(() =>
      expect(screen.getByTestId("version").textContent).toBe("Version unavailable"),
    );
  });
});
