import { describe, it, expect } from "vitest";
import { parseCommaSeparated } from "../urlParams";

describe("parseCommaSeparated", () => {
  it("splits comma-separated values", () => {
    expect(parseCommaSeparated("a,b,c")).toEqual(["a", "b", "c"]);
  });

  it("empty string returns empty array", () => {
    expect(parseCommaSeparated("")).toEqual([]);
  });

  it("filters out empty strings from consecutive commas", () => {
    expect(parseCommaSeparated("a,,b")).toEqual(["a", "b"]);
  });

  it("undefined returns empty array", () => {
    expect(parseCommaSeparated(undefined)).toEqual([]);
  });
});
