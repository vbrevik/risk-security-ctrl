import { createFileRoute, Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import {
  Network,
  ClipboardCheck,
  FileText,
  ArrowRight,
  Shield,
  GitBranch,
} from "lucide-react";
import { useEffect, useState } from "react";

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

  return (
    <div className="relative">
      {/* Background with topographic grid */}
      <div className="absolute inset-0 topo-grid gradient-mesh -z-10" />

      {/* Hero Section */}
      <section className="relative py-20 pb-12 overflow-hidden">
        {/* Decorative SVG connection lines */}
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
          {/* Framework badges */}
          <div
            className={`flex flex-wrap gap-3 mb-8 justify-center ${
              mounted ? "animate-fadeInUp delay-100" : "opacity-0"
            }`}
          >
            <span className="tech-badge">ISO 31000:2018</span>
            <span className="tech-badge">ISO 31010</span>
            <span className="tech-badge">NIST CSF</span>
            <span className="tech-badge">ISO 27000</span>
          </div>

          {/* Main heading */}
          <h1
            className={`text-5xl md:text-7xl font-bold text-center mb-6 leading-tight ${
              mounted ? "animate-fadeInUp delay-200" : "opacity-0"
            }`}
          >
            <span className="block text-foreground">
              Risk Management
            </span>
            <span className="block bg-gradient-to-r from-primary to-accent bg-clip-text text-transparent">
              Framework Explorer
            </span>
          </h1>

          {/* Description */}
          <p
            className={`text-lg md:text-xl text-muted-foreground max-w-3xl mx-auto text-center mb-12 leading-relaxed ${
              mounted ? "animate-fadeInUp delay-300" : "opacity-0"
            }`}
          >
            Navigate the complex landscape of governmental IT security through
            an ontology-first approach. Understand relationships, track
            compliance, and demonstrate adherence to international standards.
          </p>

          {/* Stats Grid */}
          <div
            className={`grid grid-cols-2 md:grid-cols-4 gap-6 max-w-4xl mx-auto mb-12 ${
              mounted ? "animate-fadeInUp delay-400" : "opacity-0"
            }`}
          >
            <StatCard number="179" label="Concepts" icon={<GitBranch className="w-5 h-5" />} />
            <StatCard number="4" label="Frameworks" icon={<Shield className="w-5 h-5" />} />
            <StatCard number="31" label="Techniques" icon={<Network className="w-5 h-5" />} />
            <StatCard number="100%" label="Compliant" icon={<ClipboardCheck className="w-5 h-5" />} />
          </div>
        </div>
      </section>

      {/* Features Section */}
      <section className="max-w-6xl mx-auto px-6 pb-20">
        <div className="grid md:grid-cols-3 gap-8">
          {/* Ontology Explorer */}
          <FeatureCard
            icon={<Network className="w-12 h-12" />}
            title={tOntology("title")}
            description="Explore risk management concepts, their relationships, and cross-framework mappings through an interactive graph visualization."
            link="/ontology"
            linkText={t("nav.ontology")}
            badge="Ontology"
            delay={500}
            mounted={mounted}
          />

          {/* Compliance Tracker */}
          <FeatureCard
            icon={<ClipboardCheck className="w-12 h-12" />}
            title={tCompliance("title")}
            description="Create assessments, track compliance status, and attach evidence to demonstrate adherence to framework requirements."
            link="/compliance"
            linkText={t("nav.compliance")}
            badge="Compliance"
            delay={600}
            mounted={mounted}
          />

          {/* Reports & Analytics */}
          <FeatureCard
            icon={<FileText className="w-12 h-12" />}
            title={tReports("title")}
            description="Generate compliance summaries, risk assessment reports, and audit trails for stakeholder communication."
            link="/reports"
            linkText={t("nav.reports")}
            badge="Reports"
            delay={700}
            mounted={mounted}
          />
        </div>
      </section>

      {/* Bottom CTA Section */}
      <section
        className={`max-w-4xl mx-auto px-6 pb-20 text-center ${
          mounted ? "animate-fadeInUp delay-800" : "opacity-0"
        }`}
      >
        <div className="border border-border rounded-lg p-8 bg-card/50 backdrop-blur-sm">
          <h2 className="text-2xl font-bold mb-4">
            Start Exploring Risk Frameworks
          </h2>
          <p className="text-muted-foreground mb-6 max-w-2xl mx-auto">
            Built for risk managers, IT specialists, and compliance officers
            who need to understand and navigate complex regulatory frameworks.
          </p>
          <Button asChild size="lg" className="group">
            <Link to="/ontology">
              Explore Ontology
              <ArrowRight className="ml-2 w-4 h-4 transition-transform group-hover:translate-x-1" />
            </Link>
          </Button>
        </div>
      </section>
    </div>
  );
}

// Stat Card Component
function StatCard({
  number,
  label,
  icon,
}: {
  number: string;
  label: string;
  icon: React.ReactNode;
}) {
  return (
    <div className="text-center p-4 rounded-lg border border-border bg-card/80 backdrop-blur-sm">
      <div className="flex items-center justify-center gap-2 mb-2 text-muted-foreground">
        {icon}
        <span className="text-sm font-medium uppercase tracking-wide">
          {label}
        </span>
      </div>
      <div className="stat-number">{number}</div>
    </div>
  );
}

// Feature Card Component
function FeatureCard({
  icon,
  title,
  description,
  link,
  linkText,
  badge,
  delay,
  mounted,
}: {
  icon: React.ReactNode;
  title: string;
  description: string;
  link: string;
  linkText: string;
  badge: string;
  delay: number;
  mounted: boolean;
}) {
  return (
    <div
      className={`feature-card corner-markers rounded-lg p-6 flex flex-col h-full ${
        mounted ? "animate-fadeInUp" : "opacity-0"
      }`}
      style={{ animationDelay: `${delay}ms` }}
    >
      <div className="flex items-start justify-between mb-4">
        <div className="text-primary">{icon}</div>
        <span className="tech-badge text-xs">{badge}</span>
      </div>

      <h3 className="text-xl font-bold mb-3">{title}</h3>

      <p className="text-muted-foreground mb-6 flex-grow leading-relaxed">
        {description}
      </p>

      <Button asChild variant="outline" className="w-full group">
        <Link to={link}>
          {linkText}
          <ArrowRight className="ml-2 w-4 h-4 transition-transform group-hover:translate-x-1" />
        </Link>
      </Button>
    </div>
  );
}
