// Tests for the version label.
import { describe, expect, it } from "vitest";
import { versionLabel } from "./version";

describe("versionLabel", () => {
  it("shows an ellipsis while loading", () => {
    expect(versionLabel({ kind: "loading" })).toBe("…");
  });

  it("prefixes a loaded version with v", () => {
    expect(versionLabel({ kind: "ready", version: "0.1.0" })).toBe("v0.1.0");
  });

  it("says when the version could not be loaded", () => {
    expect(versionLabel({ kind: "failed" })).toBe("Version unavailable");
  });
});
