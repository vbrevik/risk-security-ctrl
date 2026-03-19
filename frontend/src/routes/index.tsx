import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { useQuery, useQueries } from "@tanstack/react-query";
import { Button } from "@/components/ui/button";
import {
  Network,
  ClipboardCheck,
  FileText,
  ArrowRight,
  Shield,
  GitBranch,
  Link2,
  Layers,
} from "lucide-react";
import { useEffect, useState, useMemo } from "react";
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
  const { t } = useTranslation();
  const { t: tOntology } = useTranslation("ontology");
  const { t: tCompliance } = useTranslation("compliance");
  const { t: tReports } = useTranslation("reports");
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  // Live data
  const { data: frameworks } = useQuery({
    queryKey: ontologyKeys.frameworks(),
    queryFn: async () => {
      const { data } = await api.get<Framework[]>("/ontology/frameworks");
      return data;
    },
    staleTime: Infinity,
  });

  const { data: relationships } = useQuery({
    queryKey: ontologyKeys.relationships(),
    queryFn: async () => {
      const { data } = await api.get<Relationship[]>("/ontology/relationships");
      return data;
    },
    staleTime: Infinity,
  });

  // Fetch concept counts per framework
  const conceptQueries = useQueries({
    queries: (frameworks ?? []).map((fw) => ({
      queryKey: ontologyKeys.concepts(fw.id),
      queryFn: async () => {
        const params = new URLSearchParams();
        params.set("framework_id", fw.id);
        params.set("limit", "1"); // Just need the total count
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

  const frameworkCount = frameworks?.length ?? 0;
  const relationshipCount = relationships?.length ?? 0;

  // Group frameworks by domain for the matrix display
  const frameworkGroups = useMemo(() => {
    if (!frameworks) return [];
    const groups: { label: string; items: Framework[] }[] = [
      {
        label: "Risk & Security Standards",
        items: frameworks.filter((f) =>
          ["iso31000", "iso31010", "iso27000", "iso9000", "iso10015", "nist-csf", "nist-800-53", "nist-rmf"].includes(f.id)
        ),
      },
      {
        label: "AI Governance",
        items: frameworks.filter((f) =>
          ["eu-ai-act", "nist-ai-rmf", "iso42001", "iso42005", "iso23894", "iso24028", "google-saif", "mitre-atlas"].includes(f.id)
        ),
      },
      {
        label: "EU Regulations",
        items: frameworks.filter((f) =>
          ["gdpr", "nis2", "dora", "cer-directive"].includes(f.id)
        ),
      },
      {
        label: "Architecture & Models",
        items: frameworks.filter((f) =>
          ["zero-trust", "cisa-ztmm", "data-centric", "fmn"].includes(f.id)
        ),
      },
    ];
    return groups.filter((g) => g.items.length > 0);
  }, [frameworks]);

  // Find concept counts per framework for the framework cards
  const conceptCountMap = useMemo(() => {
    const map: Record<string, number> = {};
    for (const q of conceptQueries) {
      if (q.data) map[q.data.frameworkId] = q.data.total;
    }
    return map;
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [conceptQueries.map((q) => q.dataUpdatedAt).join(",")]);

  return (
    <div className="relative">
      {/* Background */}
      <div className="absolute inset-0 topo-grid gradient-mesh -z-10" />

      {/* Hero — typography-dominant, asymmetric */}
      <section className="relative py-16 pb-8 overflow-hidden">
        <svg
          className="absolute inset-0 w-full h-full pointer-events-none opacity-20"
          xmlns="http://www.w3.org/2000/svg"
        >
          <defs>
            <linearGradient id="lineGradient" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stopColor="var(--color-primary)" />
              <stop offset="100%" stopColor="var(--color-accent)" />
            </linearGradient>
          </defs>
          <path
            d="M 100 50 Q 300 100 500 80 T 900 120"
            stroke="url(#lineGradient)"
            strokeWidth="2"
            fill="none"
            className={mounted ? "connection-line" : ""}
            style={{ animationDelay: "0.3s" }}
          />
          <path
            d="M 200 200 Q 400 250 600 230 T 1000 270"
            stroke="url(#lineGradient)"
            strokeWidth="2"
            fill="none"
            className={mounted ? "connection-line" : ""}
            style={{ animationDelay: "0.6s" }}
          />
        </svg>

        <div className="max-w-6xl mx-auto px-6">
          {/* Heading — left-aligned, breaking the center-everything pattern */}
          <div className="max-w-4xl">
            <h1
              className={`text-5xl md:text-7xl font-bold mb-6 leading-tight ${
                mounted ? "animate-fadeInUp delay-100" : "opacity-0"
              }`}
            >
              <span className="block text-foreground">Risk &amp; Compliance</span>
              <span className="block bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
                Framework Explorer
              </span>
            </h1>

            <p
              className={`text-lg md:text-xl text-muted-foreground max-w-2xl mb-10 leading-relaxed ${
                mounted ? "animate-fadeInUp delay-200" : "opacity-0"
              }`}
            >
              Navigate {frameworkCount || "..."} interconnected regulatory frameworks,
              {" "}{totalConcepts ? totalConcepts.toLocaleString() : "..."} concepts,
              and {relationshipCount || "..."} cross-framework mappings for governmental
              IT security and AI governance.
            </p>
          </div>

          {/* Live stats — asymmetric strip */}
          <div
            className={`flex flex-wrap gap-3 mb-10 ${
              mounted ? "animate-fadeInUp delay-300" : "opacity-0"
            }`}
          >
            <LiveStat
              value={frameworkCount}
              label="Frameworks"
              icon={<Shield className="w-4 h-4" />}
            />
            <LiveStat
              value={totalConcepts}
              label="Concepts"
              icon={<GitBranch className="w-4 h-4" />}
            />
            <LiveStat
              value={relationshipCount}
              label="Cross-Mappings"
              icon={<Link2 className="w-4 h-4" />}
            />
            <LiveStat
              value={frameworkGroups.length}
              label="Domains"
              icon={<Layers className="w-4 h-4" />}
            />
          </div>
        </div>
      </section>

      {/* Framework Matrix — the "one thing someone will remember" */}
      <section className="max-w-6xl mx-auto px-6 pb-12">
        <div
          className={`${mounted ? "animate-fadeInUp delay-400" : "opacity-0"}`}
        >
          {frameworkGroups.map((group, gi) => (
            <div key={group.label} className="mb-6">
              <div className="flex items-center gap-3 mb-3">
                <h2 className="text-xs font-mono font-semibold uppercase tracking-widest text-muted-foreground">
                  {group.label}
                </h2>
                <div className="flex-1 h-px bg-border" />
                <span className="text-xs font-mono text-muted-foreground">
                  {group.items.length}
                </span>
              </div>
              <div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 gap-2">
                {group.items.map((fw, fi) => (
                  <Link
                    key={fw.id}
                    to="/ontology"
                    className="group relative border border-border rounded-md p-3 bg-card/80 backdrop-blur-sm hover:border-accent hover:-translate-y-0.5 transition-all duration-200 cursor-pointer"
                    style={{
                      animationDelay: `${400 + gi * 100 + fi * 50}ms`,
                    }}
                  >
                    <div className="flex items-start gap-2 mb-2">
                      <span
                        className="w-2 h-2 rounded-full mt-1 flex-shrink-0"
                        style={{ backgroundColor: getFrameworkColor(fw.id) }}
                      />
                      <span className="text-xs font-mono font-medium leading-tight truncate">
                        {fw.name.length > 30
                          ? fw.name.split(" - ")[0] || fw.name.substring(0, 30)
                          : fw.name}
                      </span>
                    </div>
                    <div className="flex items-baseline justify-between">
                      <span className="text-[10px] font-mono text-muted-foreground">
                        {fw.version || ""}
                      </span>
                      <span className="text-sm font-mono font-bold text-accent">
                        {conceptCountMap[fw.id] ?? "..."}
                      </span>
                    </div>
                    {/* Hover accent line */}
                    <div
                      className="absolute bottom-0 left-0 right-0 h-0.5 rounded-b-md opacity-0 group-hover:opacity-100 transition-opacity"
                      style={{ backgroundColor: getFrameworkColor(fw.id) }}
                    />
                  </Link>
                ))}
              </div>
            </div>
          ))}
        </div>
      </section>

      {/* Features — 3 cards, but asymmetric: ontology dominates */}
      <section className="max-w-6xl mx-auto px-6 pb-16">
        <div
          className={`grid md:grid-cols-5 gap-4 ${
            mounted ? "animate-fadeInUp delay-600" : "opacity-0"
          }`}
        >
          {/* Ontology Explorer — 3 cols, featured */}
          <div className="md:col-span-3 feature-card corner-markers rounded-lg p-6 flex flex-col">
            <div className="flex items-start justify-between mb-4">
              <div className="text-primary">
                <Network className="w-10 h-10" />
              </div>
              <span className="tech-badge text-xs">Primary</span>
            </div>
            <h3 className="text-2xl font-bold mb-3">{tOntology("title")}</h3>
            <p className="text-muted-foreground mb-4 flex-grow leading-relaxed">
              Explore {totalConcepts ? totalConcepts.toLocaleString() : "..."} concepts
              across {frameworkCount || "..."} frameworks with {relationshipCount || "..."} cross-framework
              mappings. Interactive graph visualization, tree navigation, and framework comparison.
            </p>
            <div className="flex flex-wrap gap-2 mb-5">
              {frameworks?.slice(0, 8).map((fw) => (
                <span
                  key={fw.id}
                  className="tech-badge text-[10px] flex items-center gap-1"
                >
                  <span
                    className="w-1.5 h-1.5 rounded-full"
                    style={{ backgroundColor: getFrameworkColor(fw.id) }}
                  />
                  {fw.id}
                </span>
              ))}
              {(frameworks?.length ?? 0) > 8 && (
                <span className="tech-badge text-[10px]">
                  +{(frameworks?.length ?? 0) - 8} more
                </span>
              )}
            </div>
            <Button asChild className="w-full group">
              <Link to="/ontology">
                {t("nav.ontology")}
                <ArrowRight className="ml-2 w-4 h-4 transition-transform group-hover:translate-x-1" />
              </Link>
            </Button>
          </div>

          {/* Right column — 2 smaller cards stacked */}
          <div className="md:col-span-2 flex flex-col gap-4">
            <div className="feature-card corner-markers rounded-lg p-5 flex-1 flex flex-col">
              <div className="flex items-start justify-between mb-3">
                <ClipboardCheck className="w-8 h-8 text-primary" />
                <span className="tech-badge text-[10px]">Track</span>
              </div>
              <h3 className="text-lg font-bold mb-2">{tCompliance("title")}</h3>
              <p className="text-sm text-muted-foreground mb-4 flex-grow leading-relaxed">
                Create assessments, track compliance status, and attach evidence
                against framework requirements.
              </p>
              <Button asChild variant="outline" size="sm" className="w-full group">
                <Link to="/compliance">
                  {t("nav.compliance")}
                  <ArrowRight className="ml-2 w-3 h-3 transition-transform group-hover:translate-x-1" />
                </Link>
              </Button>
            </div>

            <div className="feature-card corner-markers rounded-lg p-5 flex-1 flex flex-col">
              <div className="flex items-start justify-between mb-3">
                <FileText className="w-8 h-8 text-primary" />
                <span className="tech-badge text-[10px]">Report</span>
              </div>
              <h3 className="text-lg font-bold mb-2">{tReports("title")}</h3>
              <p className="text-sm text-muted-foreground mb-4 flex-grow leading-relaxed">
                Generate compliance summaries, risk assessment reports, and
                audit trails for stakeholders.
              </p>
              <Button asChild variant="outline" size="sm" className="w-full group">
                <Link to="/reports">
                  {t("nav.reports")}
                  <ArrowRight className="ml-2 w-3 h-3 transition-transform group-hover:translate-x-1" />
                </Link>
              </Button>
            </div>
          </div>
        </div>
      </section>
    </div>
  );
}

function LiveStat({
  value,
  label,
  icon,
}: {
  value: number;
  label: string;
  icon: React.ReactNode;
}) {
  return (
    <div className="flex items-center gap-2 px-4 py-2 rounded-md border border-border bg-card/80 backdrop-blur-sm">
      <span className="text-muted-foreground">{icon}</span>
      <span className="stat-number text-2xl">{value || "..."}</span>
      <span className="text-xs font-mono font-medium uppercase tracking-wide text-muted-foreground">
        {label}
      </span>
    </div>
  );
}
