import type { Framework } from "../types";

const DOMAIN_MAP: { label: string; ids: string[] }[] = [
  {
    label: "Risk & Security Standards",
    ids: ["iso31000", "iso31010", "iso27000", "iso9000", "nist-csf", "nist-800-53", "nist-rmf"],
  },
  {
    label: "AI Governance",
    ids: ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "google-saif", "mitre-atlas"],
  },
  {
    label: "EU Regulations",
    ids: ["gdpr", "nis2", "dora", "cer-directive"],
  },
  {
    label: "Architecture & Models",
    ids: ["zero-trust", "cisa-ztmm", "data-centric", "fmn"],
  },
];

export function groupFrameworksByDomain(
  frameworks: Framework[]
): { label: string; frameworkIds: string[] }[] {
  const available = new Set(frameworks.map((f) => f.id));
  return DOMAIN_MAP.map((domain) => ({
    label: domain.label,
    frameworkIds: domain.ids.filter((id) => available.has(id)),
  }));
}
