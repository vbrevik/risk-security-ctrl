import { describe, it, expect } from "vitest";
import { getFrameworkColor } from "../frameworkColors";

describe("getFrameworkColor", () => {
  it("returns a hex color string for a known framework ID", () => {
    const color = getFrameworkColor(["fw-a", "fw-b"], "fw-a");
    expect(color).toMatch(/^#[0-9a-fA-F]{6}$/);
  });

  it("same framework ID always gets same color given same frameworkIds array", () => {
    const ids = ["fw-a", "fw-b", "fw-c"];
    const color1 = getFrameworkColor(ids, "fw-b");
    const color2 = getFrameworkColor(ids, "fw-b");
    expect(color1).toBe(color2);
  });

  it("different frameworks get different colors (up to 10)", () => {
    const ids = Array.from({ length: 10 }, (_, i) => `fw-${String(i).padStart(2, "0")}`);
    const colors = ids.map((id) => getFrameworkColor(ids, id));
    expect(new Set(colors).size).toBe(10);
  });

  it("wraps around after 10 frameworks (mod 10 behavior)", () => {
    const ids = Array.from({ length: 11 }, (_, i) => `fw-${String(i).padStart(2, "0")}`);
    // After sorting alphabetically, fw-00 is index 0 and fw-10 is index 10
    const color0 = getFrameworkColor(ids, "fw-00");
    const color10 = getFrameworkColor(ids, "fw-10");
    expect(color10).toBe(color0);
  });

  it("order is deterministic (sorts IDs alphabetically before indexing)", () => {
    const color1 = getFrameworkColor(["fw-b", "fw-a"], "fw-a");
    const color2 = getFrameworkColor(["fw-a", "fw-b"], "fw-a");
    expect(color1).toBe(color2);
  });

  it("falls back to index 0 color when frameworkId not found", () => {
    const ids = ["fw-a", "fw-b"];
    const fallbackColor = getFrameworkColor(ids, "fw-unknown");
    const firstColor = getFrameworkColor(ids, "fw-a");
    expect(fallbackColor).toBe(firstColor);
  });
});
