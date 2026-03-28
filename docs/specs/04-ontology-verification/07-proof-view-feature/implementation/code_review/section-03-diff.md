diff --git a/frontend/src/features/ontology/components/VerificationBadge.tsx b/frontend/src/features/ontology/components/VerificationBadge.tsx
new file mode 100644
index 0000000..b899bb5
--- /dev/null
+++ b/frontend/src/features/ontology/components/VerificationBadge.tsx
@@ -0,0 +1,91 @@
+import { useTranslation } from "react-i18next";
+import {
+  CheckCircle2,
+  AlertTriangle,
+  Info,
+  Circle,
+  XCircle,
+} from "lucide-react";
+import { Badge } from "@/components/ui/badge";
+import { toVerificationStatus } from "../types";
+import type { VerificationStatus } from "../types";
+
+interface VerificationBadgeProps {
+  status: string | null;
+}
+
+type BadgeConfig = {
+  colorClasses: string;
+  Icon: React.ComponentType<{ className?: string; "aria-hidden"?: boolean }>;
+  i18nKey: string;
+  label: string;
+};
+
+const BADGE_CONFIG: Record<VerificationStatus | "unknown", BadgeConfig> = {
+  verified: {
+    colorClasses:
+      "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
+    Icon: CheckCircle2,
+    i18nKey: "proof.status.verified",
+    label: "Verified",
+  },
+  corrected: {
+    colorClasses:
+      "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
+    Icon: CheckCircle2,
+    i18nKey: "proof.status.corrected",
+    label: "Corrected",
+  },
+  "partially-verified": {
+    colorClasses:
+      "bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200",
+    Icon: AlertTriangle,
+    i18nKey: "proof.status.partially-verified",
+    label: "Partially Verified",
+  },
+  "structure-verified": {
+    colorClasses:
+      "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
+    Icon: Info,
+    i18nKey: "proof.status.structure-verified",
+    label: "Structure Verified",
+  },
+  unverified: {
+    colorClasses:
+      "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
+    Icon: Circle,
+    i18nKey: "proof.status.unverified",
+    label: "Unverified",
+  },
+  "needs-correction": {
+    colorClasses: "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
+    Icon: XCircle,
+    i18nKey: "proof.status.needs-correction",
+    label: "Needs Correction",
+  },
+  unknown: {
+    colorClasses:
+      "bg-gray-100 text-gray-600 dark:bg-gray-800 dark:text-gray-400",
+    Icon: Circle,
+    i18nKey: "proof.status.unknown",
+    label: "Unknown",
+  },
+};
+
+export function VerificationBadge({ status }: VerificationBadgeProps) {
+  const { t } = useTranslation("ontology");
+  const normalized = toVerificationStatus(status);
+  const config = BADGE_CONFIG[normalized];
+  const { Icon } = config;
+
+  return (
+    <Badge
+      variant="outline"
+      className={`inline-flex items-center gap-1.5 ${config.colorClasses}`}
+      aria-label={config.label}
+    >
+      <Icon className="h-3.5 w-3.5" aria-hidden />
+      <span>{t(config.i18nKey, config.label)}</span>
+    </Badge>
+  );
+}
diff --git a/frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx b/frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx
new file mode 100644
index 0000000..542a2db
--- /dev/null
+++ b/frontend/src/features/ontology/components/__tests__/VerificationBadge.test.tsx
@@ -0,0 +1,61 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import React from "react";
+import { VerificationBadge } from "../VerificationBadge";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string, fallback?: string) => fallback ?? key,
+  }),
+}));
+
+describe("VerificationBadge", () => {
+  it('renders "Verified" label for status="verified"', () => {
+    render(<VerificationBadge status="verified" />);
+    expect(screen.getByText("Verified")).toBeInTheDocument();
+  });
+
+  it('renders correct label for status="partially-verified"', () => {
+    render(<VerificationBadge status="partially-verified" />);
+    expect(screen.getByText("Partially Verified")).toBeInTheDocument();
+  });
+
+  it('renders correct label for status="structure-verified"', () => {
+    render(<VerificationBadge status="structure-verified" />);
+    expect(screen.getByText("Structure Verified")).toBeInTheDocument();
+  });
+
+  it('renders correct label for status="unverified"', () => {
+    render(<VerificationBadge status="unverified" />);
+    expect(screen.getByText("Unverified")).toBeInTheDocument();
+  });
+
+  it('renders correct label for status="needs-correction"', () => {
+    render(<VerificationBadge status="needs-correction" />);
+    expect(screen.getByText("Needs Correction")).toBeInTheDocument();
+  });
+
+  it('renders correct label for status="corrected"', () => {
+    render(<VerificationBadge status="corrected" />);
+    expect(screen.getByText("Corrected")).toBeInTheDocument();
+  });
+
+  it("renders without crashing when status is null (fallback style)", () => {
+    const { container } = render(<VerificationBadge status={null} />);
+    expect(container.firstChild).not.toBeNull();
+    expect(screen.getByText("Unknown")).toBeInTheDocument();
+  });
+
+  it("renders without crashing when status is an unknown string", () => {
+    const { container } = render(<VerificationBadge status="banana" />);
+    expect(container.firstChild).not.toBeNull();
+    expect(screen.getByText("Unknown")).toBeInTheDocument();
+  });
+
+  it("rendered element has aria-label attribute", () => {
+    const { container } = render(<VerificationBadge status="verified" />);
+    const badge = container.querySelector("[aria-label]");
+    expect(badge).not.toBeNull();
+    expect(badge?.getAttribute("aria-label")).toBeTruthy();
+  });
+});
