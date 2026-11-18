// Tests for the main pane view.
import { render, screen } from "@testing-library/svelte";
import { describe, expect, it } from "vitest";
import Main from "./Main.svelte";

describe("Main", () => {
  it("shows the empty-state hint", () => {
    render(Main);
    expect(screen.getByRole("main").textContent).toContain("Select a session.");
  });
});
