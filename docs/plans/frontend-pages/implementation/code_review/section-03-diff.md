diff --git a/docs/plans/frontend-pages/implementation/deep_implement_config.json b/docs/plans/frontend-pages/implementation/deep_implement_config.json
index 2caafc5..920e7fd 100644
--- a/docs/plans/frontend-pages/implementation/deep_implement_config.json
+++ b/docs/plans/frontend-pages/implementation/deep_implement_config.json
@@ -18,6 +18,10 @@
     "section-01-shared-infra": {
       "status": "complete",
       "commit_hash": "593e8b7"
+    },
+    "section-02-navigation": {
+      "status": "complete",
+      "commit_hash": "d03f878"
     }
   },
   "pre_commit": {
diff --git a/frontend/src/features/ontology/components/FrameworkProfile.tsx b/frontend/src/features/ontology/components/FrameworkProfile.tsx
new file mode 100644
index 0000000..43776d7
--- /dev/null
+++ b/frontend/src/features/ontology/components/FrameworkProfile.tsx
@@ -0,0 +1,272 @@
+import { useState } from "react";
+import { BookOpen, ExternalLink, ChevronRight, ChevronDown } from "lucide-react";
+import type { Framework, Concept, Relationship, FrameworkStats } from "../types";
+import { getFrameworkColor } from "../utils/graphTransform";
+
+interface FrameworkProfileProps {
+  framework: Framework | null;
+  concepts: Concept[];
+  relationships: Relationship[];
+  stats: FrameworkStats | null;
+  frameworks: Framework[];
+  isLoading: boolean;
+}
+
+function typeToHue(type: string): number {
+  let hash = 0;
+  for (const ch of type) hash = (hash * 31 + ch.charCodeAt(0)) & 0xffffff;
+  return hash % 360;
+}
+
+const REL_TYPE_COLORS: Record<string, string> = {
+  maps_to: "bg-blue-500/20 text-blue-700",
+  implements: "bg-green-500/20 text-green-700",
+  related_to: "bg-gray-500/20 text-gray-700",
+  supports: "bg-amber-500/20 text-amber-700",
+};
+
+export function FrameworkProfile({
+  framework,
+  concepts,
+  relationships,
+  stats,
+  frameworks,
+  isLoading,
+}: FrameworkProfileProps) {
+  const [expanded, setExpanded] = useState<Set<string>>(new Set());
+
+  if (!framework) {
+    return (
+      <div className="flex-1 flex flex-col items-center justify-center text-foreground/40">
+        <BookOpen className="w-12 h-12 mb-3" />
+        <p className="text-sm">Select a framework from the sidebar</p>
+      </div>
+    );
+  }
+
+  if (isLoading) {
+    return (
+      <div className="flex-1 space-y-4">
+        <div className="h-8 w-48 bg-muted rounded animate-pulse" />
+        <div className="h-4 w-96 bg-muted rounded animate-pulse" />
+        <div className="grid grid-cols-4 gap-4">
+          {Array.from({ length: 4 }).map((_, i) => (
+            <div key={i} className="h-20 bg-muted rounded animate-pulse" />
+          ))}
+        </div>
+      </div>
+    );
+  }
+
+  // Build concept-to-framework map for relationship resolution
+  const conceptFwMap = new Map<string, string>();
+  for (const c of concepts) conceptFwMap.set(c.id, c.framework_id);
+
+  // Cross-framework connections
+  const conceptIds = new Set(concepts.map((c) => c.id));
+  const fwRelationships = relationships.filter(
+    (r) => conceptIds.has(r.source_concept_id) || conceptIds.has(r.target_concept_id)
+  );
+
+  const connectionMap = new Map<string, { count: number; types: Set<string> }>();
+  for (const rel of fwRelationships) {
+    const otherConceptId = conceptIds.has(rel.source_concept_id)
+      ? rel.target_concept_id
+      : rel.source_concept_id;
+    // Try to find framework for the other concept
+    const otherFwId = conceptFwMap.get(otherConceptId);
+    if (!otherFwId || otherFwId === framework.id) continue;
+    const entry = connectionMap.get(otherFwId) ?? { count: 0, types: new Set() };
+    entry.count++;
+    entry.types.add(rel.relationship_type);
+    connectionMap.set(otherFwId, entry);
+  }
+
+  const connections = [...connectionMap.entries()]
+    .map(([fwId, data]) => ({
+      fwId,
+      name: frameworks.find((f) => f.id === fwId)?.name ?? fwId,
+      count: data.count,
+      types: [...data.types],
+    }))
+    .sort((a, b) => b.count - a.count);
+
+  // Concept hierarchy
+  const topLevel = concepts.filter((c) => c.parent_id === null);
+
+  const toggleExpand = (id: string) => {
+    setExpanded((prev) => {
+      const next = new Set(prev);
+      next.has(id) ? next.delete(id) : next.add(id);
+      return next;
+    });
+  };
+
+  return (
+    <div className="flex-1 overflow-y-auto space-y-6">
+      {/* Header */}
+      <div>
+        <div className="flex items-center gap-3 mb-2">
+          <h2 className="text-2xl font-bold font-mono">{framework.name}</h2>
+          {framework.version && (
+            <span className="tech-badge">{framework.version}</span>
+          )}
+        </div>
+        {framework.description && (
+          <p className="text-sm text-foreground/70 mb-2">{framework.description}</p>
+        )}
+        {framework.source_url && (
+          <a
+            href={framework.source_url}
+            target="_blank"
+            rel="noopener noreferrer"
+            className="inline-flex items-center gap-1 text-xs text-foreground/50 hover:text-foreground/80 transition-colors"
+          >
+            <ExternalLink className="w-3 h-3" />
+            Source
+          </a>
+        )}
+      </div>
+
+      {/* Stats Strip */}
+      {stats && (
+        <div className="grid grid-cols-4 gap-3">
+          {[
+            { label: "Concepts", value: stats.conceptCount },
+            { label: "Types", value: Object.keys(stats.conceptTypes).length },
+            { label: "Connected", value: stats.connectedFrameworks },
+            { label: "Relationships", value: stats.relationshipCount },
+          ].map((s) => (
+            <div key={s.label} className="feature-card p-3 text-center">
+              <div className="text-xs text-foreground/50 mb-1">{s.label}</div>
+              <div className="stat-number">{s.value}</div>
+            </div>
+          ))}
+        </div>
+      )}
+
+      {/* Concept Type Breakdown */}
+      {stats && Object.keys(stats.conceptTypes).length > 0 && (
+        <div>
+          <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
+            Concept Types
+          </h3>
+          <div
+            data-testid="type-breakdown-bar"
+            className="flex h-3 rounded-full overflow-hidden mb-2"
+          >
+            {Object.entries(stats.conceptTypes).map(([type, count]) => (
+              <div
+                key={type}
+                style={{
+                  width: `${(count / stats.conceptCount) * 100}%`,
+                  backgroundColor: `hsl(${typeToHue(type)}, 60%, 55%)`,
+                }}
+                title={`${type}: ${count}`}
+              />
+            ))}
+          </div>
+          <div className="flex flex-wrap gap-3 text-xs">
+            {Object.entries(stats.conceptTypes).map(([type, count]) => (
+              <div key={type} className="flex items-center gap-1.5">
+                <span
+                  className="w-2 h-2 rounded-full"
+                  style={{ backgroundColor: `hsl(${typeToHue(type)}, 60%, 55%)` }}
+                />
+                <span className="text-foreground/70 capitalize">{type}</span>
+                <span className="text-foreground/40">{count}</span>
+              </div>
+            ))}
+          </div>
+        </div>
+      )}
+
+      {/* Cross-Framework Connections */}
+      <div>
+        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
+          Cross-Framework Connections
+        </h3>
+        {connections.length === 0 ? (
+          <p className="text-xs text-foreground/40">No cross-framework connections</p>
+        ) : (
+          <div className="space-y-1">
+            {connections.map((conn) => (
+              <div
+                key={conn.fwId}
+                className="flex items-center gap-2 px-2 py-1.5 rounded hover:bg-muted/50 transition-colors text-sm"
+              >
+                <span
+                  className="w-2 h-2 rounded-full flex-shrink-0"
+                  style={{ backgroundColor: getFrameworkColor(conn.fwId) }}
+                />
+                <span className="flex-1 font-mono text-xs">{conn.name}</span>
+                <span className="tech-badge text-[10px]">{conn.count}</span>
+                {conn.types.map((t) => (
+                  <span
+                    key={t}
+                    className={`text-[9px] px-1.5 py-0.5 rounded-full ${REL_TYPE_COLORS[t] ?? "bg-gray-500/20 text-gray-700"}`}
+                  >
+                    {t.replace(/_/g, " ")}
+                  </span>
+                ))}
+              </div>
+            ))}
+          </div>
+        )}
+      </div>
+
+      {/* Concept Hierarchy Preview */}
+      <div>
+        <h3 className="text-xs font-mono uppercase tracking-widest text-foreground/50 mb-2">
+          Concept Hierarchy
+        </h3>
+        {topLevel.length === 0 ? (
+          <p className="text-xs text-foreground/40">No concepts</p>
+        ) : (
+          <div className="space-y-0.5">
+            {topLevel.map((concept) => {
+              const isExpanded = expanded.has(concept.id);
+              const children = concepts.filter((c) => c.parent_id === concept.id);
+              return (
+                <div key={concept.id}>
+                  <button
+                    onClick={() => toggleExpand(concept.id)}
+                    className="w-full flex items-center gap-1.5 px-1 py-1 rounded hover:bg-muted/50 transition-colors text-left text-xs"
+                  >
+                    {children.length > 0 ? (
+                      isExpanded ? (
+                        <ChevronDown className="w-3 h-3 text-foreground/40" />
+                      ) : (
+                        <ChevronRight className="w-3 h-3 text-foreground/40" />
+                      )
+                    ) : (
+                      <span className="w-3" />
+                    )}
+                    {concept.code && (
+                      <span className="font-mono text-foreground/50">{concept.code}</span>
+                    )}
+                    <span className="flex-1">{concept.name_en}</span>
+                    <span className="tech-badge text-[9px]">{concept.concept_type}</span>
+                  </button>
+                  {isExpanded &&
+                    children.map((child) => (
+                      <div
+                        key={child.id}
+                        className="flex items-center gap-1.5 pl-6 px-1 py-1 text-xs text-foreground/70"
+                      >
+                        {child.code && (
+                          <span className="font-mono text-foreground/40">{child.code}</span>
+                        )}
+                        <span className="flex-1">{child.name_en}</span>
+                        <span className="tech-badge text-[9px]">{child.concept_type}</span>
+                      </div>
+                    ))}
+                </div>
+              );
+            })}
+          </div>
+        )}
+      </div>
+    </div>
+  );
+}
diff --git a/frontend/src/features/ontology/components/FrameworkSidebar.tsx b/frontend/src/features/ontology/components/FrameworkSidebar.tsx
new file mode 100644
index 0000000..6902200
--- /dev/null
+++ b/frontend/src/features/ontology/components/FrameworkSidebar.tsx
@@ -0,0 +1,81 @@
+import type { Framework, FrameworkStats } from "../types";
+import { groupFrameworksByDomain } from "../utils/frameworkDomains";
+import { getFrameworkColor } from "../utils/graphTransform";
+
+interface FrameworkSidebarProps {
+  frameworks: Framework[];
+  stats: Map<string, FrameworkStats>;
+  selectedId: string | null;
+  onSelect: (id: string) => void;
+  isLoading: boolean;
+}
+
+export function FrameworkSidebar({
+  frameworks,
+  stats,
+  selectedId,
+  onSelect,
+  isLoading,
+}: FrameworkSidebarProps) {
+  if (isLoading) {
+    return (
+      <div className="w-[280px] flex-shrink-0 overflow-y-auto space-y-6">
+        {Array.from({ length: 4 }).map((_, i) => (
+          <div key={i} className="space-y-2">
+            <div className="h-3 w-32 bg-muted rounded animate-pulse" />
+            {Array.from({ length: 3 }).map((_, j) => (
+              <div key={j} className="h-8 bg-muted rounded animate-pulse" />
+            ))}
+          </div>
+        ))}
+      </div>
+    );
+  }
+
+  const groups = groupFrameworksByDomain(frameworks);
+  const frameworkById = new Map(frameworks.map((fw) => [fw.id, fw]));
+
+  return (
+    <div className="w-[280px] flex-shrink-0 overflow-y-auto space-y-4">
+      {groups.map((group) => (
+        <div key={group.label}>
+          <div className="flex items-center gap-2 mb-2">
+            <span className="text-xs font-mono uppercase tracking-widest text-foreground/50">
+              {group.label}
+            </span>
+            <span className="text-xs text-foreground/30">({group.frameworkIds.length})</span>
+          </div>
+          <div className="space-y-0.5">
+            {group.frameworkIds.map((fwId) => {
+              const fw = frameworkById.get(fwId);
+              if (!fw) return null;
+              const isActive = fwId === selectedId;
+              const count = stats.get(fwId)?.conceptCount;
+              return (
+                <button
+                  key={fwId}
+                  data-active={isActive}
+                  onClick={() => onSelect(fwId)}
+                  className={`w-full flex items-center gap-2 px-2 py-1.5 rounded text-left text-sm transition-colors ${
+                    isActive
+                      ? "bg-accent/10 border-l-2 border-accent"
+                      : "hover:bg-muted/50 border-l-2 border-transparent"
+                  }`}
+                >
+                  <span
+                    className="w-2.5 h-2.5 rounded-full flex-shrink-0"
+                    style={{ backgroundColor: getFrameworkColor(fwId) }}
+                  />
+                  <span className="flex-1 truncate font-mono text-xs">{fw.name}</span>
+                  {count != null && (
+                    <span className="tech-badge text-[10px]">{count}</span>
+                  )}
+                </button>
+              );
+            })}
+          </div>
+        </div>
+      ))}
+    </div>
+  );
+}
diff --git a/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx b/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx
new file mode 100644
index 0000000..558cfcf
--- /dev/null
+++ b/frontend/src/features/ontology/components/__tests__/FrameworkProfile.test.tsx
@@ -0,0 +1,136 @@
+import { describe, it, expect } from "vitest";
+import { render, screen } from "@testing-library/react";
+import React from "react";
+import { FrameworkProfile } from "../FrameworkProfile";
+import type { Framework, Concept, Relationship, FrameworkStats } from "../../types";
+
+const FW: Framework = {
+  id: "iso31000",
+  name: "ISO 31000",
+  version: "2018",
+  description: "Risk management guidelines",
+  source_url: "https://iso.org/31000",
+  created_at: "",
+  updated_at: "",
+};
+
+const FW_B: Framework = {
+  id: "nist-csf",
+  name: "NIST CSF",
+  version: "2.0",
+  description: null,
+  source_url: null,
+  created_at: "",
+  updated_at: "",
+};
+
+function makeConcept(id: string, type: string, parentId: string | null = null): Concept {
+  return {
+    id,
+    framework_id: "iso31000",
+    parent_id: parentId,
+    concept_type: type,
+    code: id.toUpperCase(),
+    name_en: `Concept ${id}`,
+    name_nb: null,
+    definition_en: null,
+    definition_nb: null,
+    source_reference: null,
+    sort_order: null,
+    created_at: "",
+    updated_at: "",
+  };
+}
+
+const CONCEPTS: Concept[] = [
+  makeConcept("c1", "principle"),
+  makeConcept("c2", "principle"),
+  makeConcept("c3", "process"),
+  makeConcept("c1-1", "guideline", "c1"),
+];
+
+const RELATIONSHIPS: Relationship[] = [
+  { id: "r1", source_concept_id: "c1", target_concept_id: "ext1", relationship_type: "maps_to", description: null, created_at: null },
+  { id: "r2", source_concept_id: "c2", target_concept_id: "ext1", relationship_type: "related_to", description: null, created_at: null },
+];
+
+const STATS: FrameworkStats = {
+  conceptCount: 4,
+  conceptTypes: { principle: 2, process: 1, guideline: 1 },
+  connectedFrameworks: 1,
+  relationshipCount: 2,
+};
+
+describe("FrameworkProfile", () => {
+  it("renders framework name, version, description, source link", () => {
+    render(
+      <FrameworkProfile
+        framework={FW}
+        concepts={CONCEPTS}
+        relationships={RELATIONSHIPS}
+        stats={STATS}
+        frameworks={[FW, FW_B]}
+        isLoading={false}
+      />
+    );
+    expect(screen.getByText("ISO 31000")).toBeInTheDocument();
+    expect(screen.getByText("2018")).toBeInTheDocument();
+    expect(screen.getByText("Risk management guidelines")).toBeInTheDocument();
+    const sourceLink = screen.getByRole("link", { name: /source/i });
+    expect(sourceLink).toHaveAttribute("href", "https://iso.org/31000");
+    expect(sourceLink).toHaveAttribute("target", "_blank");
+  });
+
+  it("renders 4 stat boxes with correct values", () => {
+    const { container } = render(
+      <FrameworkProfile
+        framework={FW}
+        concepts={CONCEPTS}
+        relationships={RELATIONSHIPS}
+        stats={STATS}
+        frameworks={[FW, FW_B]}
+        isLoading={false}
+      />
+    );
+    const statBoxes = container.querySelectorAll(".feature-card");
+    expect(statBoxes).toHaveLength(4);
+    const values = Array.from(statBoxes).map(
+      (box) => box.querySelector(".stat-number")?.textContent
+    );
+    expect(values).toEqual(["4", "3", "1", "2"]);
+  });
+
+  it("renders concept type breakdown bar", () => {
+    const { container } = render(
+      <FrameworkProfile
+        framework={FW}
+        concepts={CONCEPTS}
+        relationships={RELATIONSHIPS}
+        stats={STATS}
+        frameworks={[FW, FW_B]}
+        isLoading={false}
+      />
+    );
+    // Legend items for each type (use getAllByText since types appear in both legend and badges)
+    expect(screen.getAllByText(/principle/i).length).toBeGreaterThanOrEqual(1);
+    expect(screen.getAllByText(/process/i).length).toBeGreaterThanOrEqual(1);
+    expect(screen.getAllByText(/guideline/i).length).toBeGreaterThanOrEqual(1);
+    // Stacked bar should exist
+    const bar = container.querySelector("[data-testid='type-breakdown-bar']");
+    expect(bar).toBeInTheDocument();
+  });
+
+  it("renders empty state when no framework selected", () => {
+    render(
+      <FrameworkProfile
+        framework={null}
+        concepts={[]}
+        relationships={[]}
+        stats={null}
+        frameworks={[]}
+        isLoading={false}
+      />
+    );
+    expect(screen.getByText(/select a framework/i)).toBeInTheDocument();
+  });
+});
diff --git a/frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx b/frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx
new file mode 100644
index 0000000..579d837
--- /dev/null
+++ b/frontend/src/features/ontology/components/__tests__/FrameworkSidebar.test.tsx
@@ -0,0 +1,110 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen } from "@testing-library/react";
+import userEvent from "@testing-library/user-event";
+import React from "react";
+import { FrameworkSidebar } from "../FrameworkSidebar";
+import type { Framework, FrameworkStats } from "../../types";
+
+function makeFramework(id: string, name: string): Framework {
+  return { id, name, version: "1.0", description: null, source_url: null, created_at: "", updated_at: "" };
+}
+
+const FRAMEWORKS: Framework[] = [
+  makeFramework("iso31000", "ISO 31000"),
+  makeFramework("iso31010", "ISO 31010"),
+  makeFramework("eu-ai-act", "EU AI Act"),
+  makeFramework("gdpr", "GDPR"),
+  makeFramework("zero-trust", "Zero Trust"),
+];
+
+const STATS = new Map<string, FrameworkStats>([
+  ["iso31000", { conceptCount: 42, conceptTypes: { principle: 10, process: 32 }, connectedFrameworks: 3, relationshipCount: 15 }],
+  ["iso31010", { conceptCount: 28, conceptTypes: { technique: 28 }, connectedFrameworks: 2, relationshipCount: 8 }],
+  ["eu-ai-act", { conceptCount: 55, conceptTypes: { requirement: 55 }, connectedFrameworks: 4, relationshipCount: 20 }],
+  ["gdpr", { conceptCount: 30, conceptTypes: { article: 30 }, connectedFrameworks: 1, relationshipCount: 5 }],
+  ["zero-trust", { conceptCount: 18, conceptTypes: { pillar: 18 }, connectedFrameworks: 2, relationshipCount: 10 }],
+]);
+
+describe("FrameworkSidebar", () => {
+  it("renders all frameworks grouped by domain", () => {
+    render(
+      <FrameworkSidebar
+        frameworks={FRAMEWORKS}
+        stats={STATS}
+        selectedId={null}
+        onSelect={vi.fn()}
+        isLoading={false}
+      />
+    );
+    // Should render domain headings
+    expect(screen.getByText(/Risk & Security/i)).toBeInTheDocument();
+    expect(screen.getByText(/AI Governance/i)).toBeInTheDocument();
+    expect(screen.getByText(/EU Regulations/i)).toBeInTheDocument();
+    expect(screen.getByText(/Architecture/i)).toBeInTheDocument();
+
+    // Should render framework names
+    expect(screen.getByText("ISO 31000")).toBeInTheDocument();
+    expect(screen.getByText("EU AI Act")).toBeInTheDocument();
+    expect(screen.getByText("GDPR")).toBeInTheDocument();
+    expect(screen.getByText("Zero Trust")).toBeInTheDocument();
+  });
+
+  it("each framework shows concept count", () => {
+    render(
+      <FrameworkSidebar
+        frameworks={FRAMEWORKS}
+        stats={STATS}
+        selectedId={null}
+        onSelect={vi.fn()}
+        isLoading={false}
+      />
+    );
+    expect(screen.getByText("42")).toBeInTheDocument();
+    expect(screen.getByText("55")).toBeInTheDocument();
+  });
+
+  it("clicking a framework calls the selection callback", async () => {
+    const onSelect = vi.fn();
+    const user = userEvent.setup();
+    render(
+      <FrameworkSidebar
+        frameworks={FRAMEWORKS}
+        stats={STATS}
+        selectedId={null}
+        onSelect={onSelect}
+        isLoading={false}
+      />
+    );
+    await user.click(screen.getByText("GDPR"));
+    expect(onSelect).toHaveBeenCalledWith("gdpr");
+  });
+
+  it("active framework is visually highlighted", () => {
+    const { container } = render(
+      <FrameworkSidebar
+        frameworks={FRAMEWORKS}
+        stats={STATS}
+        selectedId="iso31000"
+        onSelect={vi.fn()}
+        isLoading={false}
+      />
+    );
+    const activeItem = container.querySelector("[data-active='true']");
+    expect(activeItem).toBeInTheDocument();
+    expect(activeItem?.textContent).toContain("ISO 31000");
+  });
+
+  it("renders loading skeleton when data is pending", () => {
+    const { container } = render(
+      <FrameworkSidebar
+        frameworks={[]}
+        stats={new Map()}
+        selectedId={null}
+        onSelect={vi.fn()}
+        isLoading={true}
+      />
+    );
+    const skeletons = container.querySelectorAll(".animate-pulse");
+    expect(skeletons.length).toBeGreaterThan(0);
+  });
+});
diff --git a/frontend/src/routes/frameworks/index.tsx b/frontend/src/routes/frameworks/index.tsx
index 7f9311c..35225f4 100644
--- a/frontend/src/routes/frameworks/index.tsx
+++ b/frontend/src/routes/frameworks/index.tsx
@@ -1,4 +1,8 @@
+import { useEffect } from "react";
 import { createFileRoute } from "@tanstack/react-router";
+import { useFrameworks, useConcepts, useRelationships, useFrameworkStats } from "@/features/ontology/api";
+import { FrameworkSidebar } from "@/features/ontology/components/FrameworkSidebar";
+import { FrameworkProfile } from "@/features/ontology/components/FrameworkProfile";
 
 export const Route = createFileRoute("/frameworks/")({
   component: FrameworkCatalogPage,
@@ -8,9 +12,46 @@ export const Route = createFileRoute("/frameworks/")({
 });
 
 function FrameworkCatalogPage() {
+  const { id } = Route.useSearch();
+  const navigate = Route.useNavigate();
+  const { data: frameworks = [], isLoading: fwLoading } = useFrameworks();
+  const { data: statsMap, isLoading: statsLoading } = useFrameworkStats();
+  const { data: relationships = [] } = useRelationships();
+
+  // Auto-select first framework if no ?id
+  useEffect(() => {
+    if (fwLoading || frameworks.length === 0) return;
+    if (!id) {
+      navigate({ search: { id: frameworks[0].id }, replace: true });
+    } else if (!frameworks.find((fw) => fw.id === id)) {
+      navigate({ search: { id: frameworks[0].id }, replace: true });
+    }
+  }, [id, frameworks, fwLoading, navigate]);
+
+  const selectedId = id ?? frameworks[0]?.id ?? null;
+  const selectedFramework = frameworks.find((fw) => fw.id === selectedId) ?? null;
+  const { data: concepts = [], isLoading: conceptsLoading } = useConcepts(selectedId ?? undefined);
+
   return (
-    <div>
-      <h1 className="text-2xl font-bold font-mono">Framework Catalog</h1>
+    <div className="animate-fadeInUp">
+      <h1 className="text-2xl font-bold font-mono mb-6">Framework Catalog</h1>
+      <div className="flex gap-6 h-[calc(100vh-12rem)]">
+        <FrameworkSidebar
+          frameworks={frameworks}
+          stats={statsMap}
+          selectedId={selectedId}
+          onSelect={(fwId) => navigate({ search: { id: fwId } })}
+          isLoading={fwLoading || statsLoading}
+        />
+        <FrameworkProfile
+          framework={selectedFramework}
+          concepts={concepts}
+          relationships={relationships}
+          stats={selectedId ? statsMap.get(selectedId) ?? null : null}
+          frameworks={frameworks}
+          isLoading={conceptsLoading}
+        />
+      </div>
     </div>
   );
 }
