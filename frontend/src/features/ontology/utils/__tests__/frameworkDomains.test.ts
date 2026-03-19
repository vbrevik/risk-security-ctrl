import { describe, it, expect } from "vitest";
import { groupFrameworksByDomain } from "../frameworkDomains";
import type { Framework } from "../../types";

function makeFramework(id: string): Framework {
  return {
    id,
    name: id,
    version: null,
    description: null,
    source_url: null,
    created_at: "",
    updated_at: "",
  };
}

const ALL_IDS = [
  "iso31000", "iso31010", "iso27000", "iso9000", "iso10015", "nist-csf", "nist-800-53", "nist-rmf",
  "eu-ai-act", "nist-ai-rmf", "nist-ai-genai", "iso42001", "iso42005", "iso23894", "iso24028", "google-saif", "mitre-atlas",
  "gdpr", "nis2", "dora", "cer-directive",
  "zero-trust", "cisa-ztmm", "data-centric", "fmn",
];

describe("groupFrameworksByDomain", () => {
  it("returns 4 groups with correct labels", () => {
    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
    expect(groups).toHaveLength(4);
    expect(groups.map((g) => g.label)).toEqual([
      "Risk & Security Standards",
      "AI Governance",
      "EU Regulations",
      "Architecture & Models",
    ]);
  });

  it("each group contains expected framework IDs", () => {
    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
    const byLabel = Object.fromEntries(groups.map((g) => [g.label, g.frameworkIds]));

    expect(byLabel["Risk & Security Standards"]).toEqual(
      expect.arrayContaining(["iso31000", "iso31010", "iso27000", "iso9000", "iso10015", "nist-csf", "nist-800-53", "nist-rmf"])
    );
    expect(byLabel["AI Governance"]).toEqual(
      expect.arrayContaining(["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "iso24028", "google-saif", "mitre-atlas"])
    );
    expect(byLabel["EU Regulations"]).toEqual(
      expect.arrayContaining(["gdpr", "nis2", "dora", "cer-directive"])
    );
    expect(byLabel["Architecture & Models"]).toEqual(
      expect.arrayContaining(["zero-trust", "cisa-ztmm", "data-centric", "fmn"])
    );
  });

  it("all 22 frameworks assigned to exactly one group", () => {
    const groups = groupFrameworksByDomain(ALL_IDS.map(makeFramework));
    const allIds = groups.flatMap((g) => g.frameworkIds);
    expect(allIds).toHaveLength(25);
    expect(new Set(allIds).size).toBe(25);
  });

  it("handles empty framework array", () => {
    const groups = groupFrameworksByDomain([]);
    expect(groups).toHaveLength(4);
    groups.forEach((g) => {
      expect(g.frameworkIds).toEqual([]);
    });
  });

  it("unknown framework IDs excluded", () => {
    const groups = groupFrameworksByDomain([makeFramework("unknown-fw")]);
    const allIds = groups.flatMap((g) => g.frameworkIds);
    expect(allIds).not.toContain("unknown-fw");
  });
});
