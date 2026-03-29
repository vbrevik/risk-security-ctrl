import { useMemo } from "react";
import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useFrameworkProof } from "../api";
import { VerificationBadge } from "./VerificationBadge";

interface ProofPanelProps {
  frameworkId: string;
}

/**
 * Displays verification provenance for a framework.
 * Fetches proof data lazily when mounted.
 * Renders: loading skeleton | error message | metadata + optional markdown.
 */
export function ProofPanel({ frameworkId }: ProofPanelProps) {
  const { t } = useTranslation("ontology");
  const { data, isLoading, isError } = useFrameworkProof(frameworkId);

  if (isLoading) {
    return (
      <div className="space-y-2 p-4">
        <div className="h-4 w-full bg-muted rounded animate-pulse" />
        <div className="h-4 w-3/4 bg-muted rounded animate-pulse" />
        <div className="h-4 w-1/2 bg-muted rounded animate-pulse" />
      </div>
    );
  }

  if (isError || !data) {
    return (
      <div className="p-4 text-sm text-muted-foreground">
        {t("proof.error", "Could not load proof document.")}
      </div>
    );
  }

  const formattedDate = data.verification_date
    ? new Date(data.verification_date).toLocaleDateString()
    : null;

  return (
    <div className="p-4 space-y-3 text-sm">
      {/* Metadata row */}
      <div className="flex flex-wrap items-center gap-3">
        <VerificationBadge status={data.verification_status} />
        {data.source_trust_tier != null && (
          <SourceTrustBadge tier={data.source_trust_tier} />
        )}
        {formattedDate && (
          <span className="text-muted-foreground">
            {t("proof.date", "Verified")}: {formattedDate}
          </span>
        )}
        {data.verification_source && (
          <a
            href={data.verification_source}
            target="_blank"
            rel="noopener noreferrer"
            className="inline-flex items-center gap-1 text-muted-foreground hover:text-foreground"
          >
            <ExternalLink className="w-3 h-3" />
            {t("proof.source", "Source")}
          </a>
        )}
      </div>
      {data.verification_notes && (
        <p className="text-muted-foreground">
          <span className="font-medium">{t("proof.notes", "Notes")}: </span>
          {data.verification_notes}
        </p>
      )}

      {data.proof_content ? (
        <>
          <hr className="border-border" />
          <MarkdownContent content={data.proof_content} />
        </>
      ) : (
        <p className="text-muted-foreground italic">
          {t("proof.noProof", "No proof document available")}
        </p>
      )}
    </div>
  );
}

const TRUST_CONFIG: Record<
  1 | 2 | 3,
  { label: string; i18nKey: string; colorClasses: string; title: string }
> = {
  1: {
    label: "Primary Source",
    i18nKey: "proof.trustTier.primary",
    colorClasses:
      "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
    title: "Verified against official primary source (EUR-Lex, NIST, MITRE, ISO, government sites)",
  },
  2: {
    label: "Secondary Source",
    i18nKey: "proof.trustTier.secondary",
    colorClasses:
      "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
    title: "Verified via legitimate secondary source (official distributors, structured readers)",
  },
  3: {
    label: "Unofficial Source",
    i18nKey: "proof.trustTier.unofficial",
    colorClasses:
      "bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200",
    title: "Verified using unofficial source (unauthorized PDF copy or compliance vendor summary)",
  },
};

function SourceTrustBadge({ tier }: { tier: number }) {
  const { t } = useTranslation("ontology");
  const config = TRUST_CONFIG[tier as 1 | 2 | 3];
  if (!config) return null;
  return (
    <span
      className={`inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold ${config.colorClasses}`}
      title={config.title}
      aria-label={t(config.i18nKey, config.label)}
    >
      {t(config.i18nKey, config.label)}
    </span>
  );
}

function MarkdownContent({ content }: { content: string }) {
  const rendered = useMemo(
    () => (
      <ReactMarkdown remarkPlugins={[remarkGfm]}>{content}</ReactMarkdown>
    ),
    [content]
  );

  return (
    <div className="max-h-96 overflow-y-auto prose prose-sm dark:prose-invert">
      {rendered}
    </div>
  );
}
