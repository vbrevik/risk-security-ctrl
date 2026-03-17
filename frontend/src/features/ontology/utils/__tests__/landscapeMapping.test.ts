import { describe, it, expect } from "vitest";
import { getApplicableFrameworks } from "../landscapeMapping";

const UNIVERSALS = ["iso31000", "iso31010", "iso9000"];

describe("getApplicableFrameworks", () => {
  it("Financial sector returns base frameworks", () => {
    const result = getApplicableFrameworks("Financial", []);
    expect(result).toEqual(expect.arrayContaining([...UNIVERSALS, "dora", "nis2", "iso27000", "gdpr"]));
  });

  it("Deploying AI systems activity adds correct frameworks", () => {
    const result = getApplicableFrameworks("", ["Deploying AI systems"]);
    expect(result).toEqual(
      expect.arrayContaining(["eu-ai-act", "nist-ai-rmf", "iso42001", "iso23894"])
    );
  });

  it("combined sector + activities produces no duplicates", () => {
    const result = getApplicableFrameworks("Financial", ["Financial services"]);
    const uniqueCount = new Set(result).size;
    expect(result).toHaveLength(uniqueCount);
    expect(result).toContain("dora");
  });

  it("universal frameworks always included", () => {
    const result = getApplicableFrameworks("Healthcare", ["Processing personal data"]);
    UNIVERSALS.forEach((fw) => {
      expect(result).toContain(fw);
    });
  });

  it("empty sector + no activities returns only universals", () => {
    const result = getApplicableFrameworks("", []);
    expect(result.sort()).toEqual([...UNIVERSALS].sort());
  });
});
