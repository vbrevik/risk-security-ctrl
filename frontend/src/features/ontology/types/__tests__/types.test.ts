import { describe, expect, it } from "vitest";
import { toVerificationStatus } from "../index";

describe("toVerificationStatus", () => {
  it("returns 'unknown' for null input", () => {
    expect(toVerificationStatus(null)).toBe("unknown");
  });

  it("returns 'verified' for 'verified' input", () => {
    expect(toVerificationStatus("verified")).toBe("verified");
  });

  it("returns 'unknown' for an unrecognized string like 'banana'", () => {
    expect(toVerificationStatus("banana")).toBe("unknown");
  });

  it("returns 'partially-verified' for 'partially-verified' input", () => {
    expect(toVerificationStatus("partially-verified")).toBe("partially-verified");
  });

  it("returns 'needs-correction' for 'needs-correction' input", () => {
    expect(toVerificationStatus("needs-correction")).toBe("needs-correction");
  });

  it("returns 'structure-verified' for 'structure-verified' input", () => {
    expect(toVerificationStatus("structure-verified")).toBe("structure-verified");
  });

  it("returns 'corrected' for 'corrected' input", () => {
    expect(toVerificationStatus("corrected")).toBe("corrected");
  });

  it("returns 'unverified' for 'unverified' input", () => {
    expect(toVerificationStatus("unverified")).toBe("unverified");
  });

  it("returns 'unknown' for empty string", () => {
    expect(toVerificationStatus("")).toBe("unknown");
  });

  it("returns 'unknown' for undefined (missing JSON key scenario)", () => {
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    expect(toVerificationStatus(undefined as any)).toBe("unknown");
  });
});
