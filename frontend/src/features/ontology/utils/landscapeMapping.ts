export const UNIVERSAL_FRAMEWORKS = ["iso31000", "iso31010", "iso9000"];

export const SECTORS = [
  { key: "Financial", label: "Financial", description: "Banking, insurance, investment" },
  { key: "Healthcare", label: "Healthcare", description: "Hospitals, medtech, pharma" },
  { key: "Critical Infrastructure", label: "Critical Infrastructure", description: "Energy, transport, water" },
  { key: "Government/Public Admin", label: "Government/Public Admin", description: "Public sector, agencies" },
  { key: "Technology/AI Provider", label: "Technology/AI Provider", description: "Tech companies, AI developers" },
  { key: "General Enterprise", label: "General Enterprise", description: "General business operations" },
];

export const ACTIVITIES = [
  { key: "Processing personal data", label: "Processing personal data" },
  { key: "Deploying AI systems", label: "Deploying AI systems" },
  { key: "Operating critical infrastructure", label: "Operating critical infrastructure" },
  { key: "Financial services", label: "Financial services" },
  { key: "Defense/NATO context", label: "Defense/NATO context" },
];

const SECTOR_FRAMEWORKS: Record<string, string[]> = {
  Financial: ["dora", "nis2", "iso27000", "gdpr"],
  Healthcare: ["nis2", "gdpr", "iso27000"],
  "Critical Infrastructure": ["nis2", "cer-directive", "iso27000", "nist-csf"],
  "Government/Public Admin": ["nis2", "gdpr", "iso27000"],
  "Technology/AI Provider": ["gdpr", "iso27000"],
  "General Enterprise": ["iso27000", "gdpr"],
};

const ACTIVITY_FRAMEWORKS: Record<string, string[]> = {
  "Processing personal data": ["gdpr"],
  "Deploying AI systems": ["eu-ai-act", "nist-ai-rmf", "nist-ai-genai", "iso42001", "iso23894"],
  "Operating critical infrastructure": ["cer-directive", "nist-csf"],
  "Financial services": ["dora"],
  "Defense/NATO context": ["fmn", "zero-trust", "cisa-ztmm"],
};

export function getApplicableFrameworks(
  sector: string,
  activities: string[]
): string[] {
  const result = new Set(UNIVERSAL_FRAMEWORKS);

  const sectorFws = SECTOR_FRAMEWORKS[sector];
  if (sectorFws) {
    sectorFws.forEach((fw) => result.add(fw));
  }

  for (const activity of activities) {
    const activityFws = ACTIVITY_FRAMEWORKS[activity];
    if (activityFws) {
      activityFws.forEach((fw) => result.add(fw));
    }
  }

  return [...result];
}
