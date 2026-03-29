import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useQuery, useQueries } from "@tanstack/react-query";
import { useAuth } from "@/features/auth/useAuth";
import { Button } from "@/components/ui/button";
import {
  Network,
  ClipboardCheck,
  FileText,
  ArrowRight,
  Shield,
  GitBranch,
  Link2,
  Search,
  ChevronRight,
} from "lucide-react";
import { useMemo } from "react";
import { api } from "@/lib/api";
import type {
  Framework,
  Concept,
  Relationship,
  PaginatedResponse,
} from "@/features/ontology/types";
import { ontologyKeys } from "@/features/ontology/api";
import { getFrameworkColor } from "@/features/ontology/utils/graphTransform";

export const Route = createFileRoute("/")({
  component: HomePage,
});

function HomePage() {
  useTranslation();
  const { isAuthenticated } = useAuth();

  const { data: frameworks } = useQuery({
    queryKey: ontologyKeys.frameworks(),
    queryFn: async () => {
      const { data } = await api.get<Framework[]>("/ontology/frameworks");
      return data;
    },
    staleTime: Infinity,
    enabled: import.meta.env.DEV || isAuthenticated,
  });

  const { data: relationships } = useQuery({
    queryKey: ontologyKeys.relationships(),
    queryFn: async () => {
      const { data } = await api.get<Relationship[]>("/ontology/relationships");
      return data;
    },
    staleTime: Infinity,
    enabled: import.meta.env.DEV || isAuthenticated,
  });

  const conceptQueries = useQueries({
    queries: (frameworks ?? []).map((fw) => ({
      queryKey: ontologyKeys.concepts(fw.id),
      queryFn: async () => {
        const params = new URLSearchParams();
        params.set("framework_id", fw.id);
        params.set("limit", "1");
        const { data } = await api.get<PaginatedResponse<Concept>>(
          `/ontology/concepts?${params}`
        );
        return { frameworkId: fw.id, total: data.total };
      },
      staleTime: 1000 * 60 * 5,
    })),
  });

  const totalConcepts = useMemo(
    () => conceptQueries.reduce((sum, q) => sum + (q.data?.total ?? 0), 0),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [conceptQueries.map((q) => q.dataUpdatedAt).join(",")]
  );

  const conceptCountMap = useMemo(() => {
    const map: Record<string, number> = {};
    for (const q of conceptQueries) {
      if (q.data) map[q.data.frameworkId] = q.data.total;
    }
    return map;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [conceptQueries.map((q) => q.dataUpdatedAt).join(",")]);

  const frameworkCount = frameworks?.length ?? 0;
  const relationshipCount = relationships?.length ?? 0;

  const frameworkGroups = useMemo(() => {
    if (!frameworks) return [];
    const groups: { label: string; items: Framework[] }[] = [
      {
        label: "Risk & Security Standards",
        items: frameworks.filter((f) =>
          ["iso31000", "iso31010", "iso27000", "iso9000", "iso10015", "nist-csf", "nist-sp-800-53", "nist-rmf", "nist-ssdf", "nist-c-scrm", "nsm-grunnprinsipper", "nsl-sikkerhetsloven"].includes(f.id)
        ),
      },
      {
        label: "AI Governance",
        items: frameworks.filter((f) =>
          ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "iso24028", "google-saif", "mitre-atlas"].includes(f.id)
        ),
      },
      {
        label: "EU & Privacy Regulations",
        items: frameworks.filter((f) =>
          ["gdpr", "nis2", "dora", "cer-directive", "nist-privacy-framework"].includes(f.id)
        ),
      },
      {
        label: "Architecture & Threat Models",
        items: frameworks.filter((f) =>
          ["zero-trust", "cisa-ztmm", "data-centric", "fmn", "mitre-attack", "cve-cwe", "xai-dataops"].includes(f.id)
        ),
      },
    ];
    return groups.filter((g) => g.items.length > 0);
  }, [frameworks]);

  return (
    <div className="min-h-screen">

      {/* ── Purpose strip ─────────────────────────────────────────────────── */}
      <div className="border-b border-border bg-card/50">
        <div className="max-w-6xl mx-auto px-6 py-4 flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <p className="text-sm text-muted-foreground">
            Explore and map {frameworkCount || "—"} regulatory frameworks across{" "}
            {totalConcepts ? totalConcepts.toLocaleString() : "—"} concepts with{" "}
            {relationshipCount || "—"} cross-framework relationships.
          </p>
          <div className="flex items-center gap-4 shrink-0">
            <StatPill value={frameworkCount} label="Frameworks" icon={<Shield className="w-3 h-3" />} />
            <StatPill value={totalConcepts} label="Concepts" icon={<GitBranch className="w-3 h-3" />} />
            <StatPill value={relationshipCount} label="Mappings" icon={<Link2 className="w-3 h-3" />} />
          </div>
        </div>
      </div>

      {/* ── Process flow ──────────────────────────────────────────────────── */}
      <section className="max-w-6xl mx-auto px-6 pt-10 pb-8">
        <h2 className="text-xs font-mono font-semibold uppercase tracking-widest text-muted-foreground mb-6">
          How to use this tool
        </h2>

        <div className="grid md:grid-cols-3 gap-px bg-border rounded-lg overflow-hidden">
          <ProcessStep
            number="01"
            icon={<Network className="w-5 h-5" />}
            title="Explore the ontology"
            description="Browse frameworks, drill into concepts, and trace cross-framework relationships in the interactive graph."
            cta="Open Ontology Explorer"
            to="/ontology"
            primary
          />
          <ProcessStep
            number="02"
            icon={<ClipboardCheck className="w-5 h-5" />}
            title="Run an assessment"
            description="Select a framework and create a compliance checklist. Track status and attach evidence against each requirement."
            cta="Start an Assessment"
            to="/compliance"
          />
          <ProcessStep
            number="03"
            icon={<FileText className="w-5 h-5" />}
            title="Generate a report"
            description="Export compliance summaries, gap analyses, and audit trails for stakeholders and review boards."
            cta="View Reports"
            to="/reports"
          />
        </div>
      </section>

      {/* ── Quick search shortcut ─────────────────────────────────────────── */}
      <div className="max-w-6xl mx-auto px-6 pb-10">
        <Link
          to="/ontology"
          className="flex items-center gap-3 px-4 py-3 rounded-md border border-border bg-card/60 hover:bg-card hover:border-accent/50 transition-all duration-150 group w-full"
        >
          <Search className="w-4 h-4 text-muted-foreground shrink-0" />
          <span className="text-sm text-muted-foreground group-hover:text-foreground transition-colors">
            Search for a concept, control, or requirement…
          </span>
          <span className="ml-auto text-xs font-mono text-muted-foreground bg-muted px-1.5 py-0.5 rounded shrink-0">
            ⌘K
          </span>
        </Link>
      </div>

      {/* ── Framework landscape ───────────────────────────────────────────── */}
      <section className="max-w-6xl mx-auto px-6 pb-16">
        <div className="flex items-center gap-3 mb-6">
          <h2 className="text-xs font-mono font-semibold uppercase tracking-widest text-muted-foreground">
            Framework Landscape
          </h2>
          <div className="flex-1 h-px bg-border" />
          <span className="text-xs font-mono text-muted-foreground">{frameworkCount} frameworks</span>
        </div>

        <div className="space-y-6">
          {frameworkGroups.map((group) => (
            <div key={group.label}>
              <div className="flex items-center gap-2 mb-2">
                <span className="text-[11px] font-mono text-muted-foreground/70 uppercase tracking-wider">
                  {group.label}
                </span>
                <ChevronRight className="w-3 h-3 text-muted-foreground/40" />
                <span className="text-[11px] font-mono text-muted-foreground/50">
                  {group.items.length}
                </span>
              </div>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-2">
                {group.items.map((fw) => (
                  <FrameworkTile
                    key={fw.id}
                    framework={fw}
                    conceptCount={conceptCountMap[fw.id]}
                  />
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>
    </div>
  );
}

// ── Sub-components ────────────────────────────────────────────────────────────

function StatPill({
  value,
  label,
  icon,
}: {
  value: number;
  label: string;
  icon: React.ReactNode;
}) {
  return (
    <div className="flex items-center gap-1.5 text-xs font-mono text-muted-foreground">
      <span className="opacity-60">{icon}</span>
      <span className="font-semibold text-foreground tabular-nums">
        {value ? value.toLocaleString() : "—"}
      </span>
      <span className="hidden sm:inline">{label}</span>
    </div>
  );
}

function ProcessStep({
  number,
  icon,
  title,
  description,
  cta,
  to,
  primary = false,
}: {
  number: string;
  icon: React.ReactNode;
  title: string;
  description: string;
  cta: string;
  to: string;
  primary?: boolean;
}) {
  return (
    <div
      className={`flex flex-col p-6 ${
        primary ? "bg-card" : "bg-card/50"
      } hover:bg-card transition-colors duration-150`}
    >
      <div className="flex items-start justify-between mb-4">
        <span className="text-4xl font-mono font-bold text-border select-none leading-none">
          {number}
        </span>
        <span className={primary ? "text-primary" : "text-muted-foreground"}>
          {icon}
        </span>
      </div>
      <h3 className="font-semibold text-base mb-2 leading-snug">{title}</h3>
      <p className="text-sm text-muted-foreground leading-relaxed flex-1 mb-5">
        {description}
      </p>
      <Button
        asChild
        variant={primary ? "default" : "outline"}
        size="sm"
        className="w-full group"
      >
        <Link to={to}>
          {cta}
          <ArrowRight className="ml-2 w-3.5 h-3.5 transition-transform group-hover:translate-x-0.5" />
        </Link>
      </Button>
    </div>
  );
}

function FrameworkTile({
  framework,
  conceptCount,
}: {
  framework: Framework;
  conceptCount?: number;
}) {
  const color = getFrameworkColor(framework.id);
  const shortName =
    framework.name.length > 28
      ? framework.name.split(" - ")[0]?.substring(0, 28) ?? framework.name.substring(0, 28)
      : framework.name;

  return (
    <Link
      to="/ontology"
      className="group relative flex flex-col justify-between border border-border rounded-md p-3 bg-card/70 hover:bg-card hover:border-accent/40 hover:-translate-y-px transition-all duration-150 cursor-pointer min-h-[72px]"
    >
      <div className="flex items-start gap-1.5 mb-2">
        <span
          className="w-1.5 h-1.5 rounded-full mt-[3px] shrink-0"
          style={{ backgroundColor: color }}
        />
        <span className="text-[11px] font-mono font-medium leading-tight text-foreground/80 group-hover:text-foreground transition-colors">
          {shortName}
        </span>
      </div>
      <div className="flex items-baseline justify-between">
        <span className="text-[10px] font-mono text-muted-foreground/60">
          {framework.version ?? ""}
        </span>
        <span className="text-xs font-mono font-bold tabular-nums" style={{ color }}>
          {conceptCount ?? "·"}
        </span>
      </div>
      <div
        className="absolute bottom-0 left-0 right-0 h-[2px] rounded-b-md scale-x-0 group-hover:scale-x-100 transition-transform duration-200 origin-left"
        style={{ backgroundColor: color }}
      />
    </Link>
  );
}
