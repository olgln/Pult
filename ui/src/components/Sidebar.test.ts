// Tests for the sidebar view.
import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";
import Sidebar from "./Sidebar.svelte";

describe("Sidebar", () => {
  it("is a labelled region with the app name and the version", () => {
    render(Sidebar, { version: { kind: "ready", version: "0.1.0" } });
    expect(screen.getByRole("complementary", { name: "Sessions" })).toBeTruthy();
    expect(screen.getByText("Pult")).toBeTruthy();
    expect(screen.getByTestId("version").textContent).toBe("v0.1.0");
  });
});
