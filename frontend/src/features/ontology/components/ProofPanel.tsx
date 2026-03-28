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
