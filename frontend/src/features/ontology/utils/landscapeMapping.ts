const UNIVERSAL_FRAMEWORKS = ["iso31000", "iso31010", "iso9000"];

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
  "Deploying AI systems": ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso23894"],
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
