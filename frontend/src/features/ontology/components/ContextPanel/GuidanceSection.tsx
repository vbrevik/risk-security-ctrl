import { ExternalLink } from "lucide-react";
import { useTranslation } from "react-i18next";
import type { ConceptGuidanceResponse } from "../../types";

interface GuidanceSectionProps {
  guidance: ConceptGuidanceResponse;
  language: "en" | "nb";
}

export function GuidanceSection({ guidance, language }: GuidanceSectionProps) {
  const { t } = useTranslation("ontology");

  return (
    <>
      {/* Suggested Actions */}
      {guidance.suggested_actions.length > 0 && (
        <div>
          <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
            {t("guidance.suggestedActions")}
            <span className="ml-1 font-normal">({guidance.suggested_actions.length})</span>
          </h3>
          <ul className="space-y-1">
            {guidance.suggested_actions.map((action) => (
              <li key={action.sort_order} className="text-xs leading-relaxed flex gap-1.5">
                <span className="text-muted-foreground flex-shrink-0 mt-0.5">&bull;</span>
                <span>
                  {language === "nb" && action.text_nb ? action.text_nb : action.text_en}
                </span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* Transparency Questions */}
      {guidance.transparency_questions.length > 0 && (
        <div>
          <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
            {t("guidance.transparencyQuestions")}
            <span className="ml-1 font-normal">({guidance.transparency_questions.length})</span>
          </h3>
          <ul className="space-y-1">
            {guidance.transparency_questions.map((question) => (
              <li key={question.sort_order} className="text-xs leading-relaxed flex gap-1.5">
                <span className="text-muted-foreground flex-shrink-0 mt-0.5">&bull;</span>
                <span>
                  {language === "nb" && question.text_nb ? question.text_nb : question.text_en}
                </span>
              </li>
            ))}
          </ul>
        </div>
      )}

      {/* References */}
      {guidance.references.length > 0 && (
        <div>
          <h3 className="font-medium text-xs text-muted-foreground mb-1.5 uppercase tracking-wide">
            {t("guidance.references")}
            <span className="ml-1 font-normal">({guidance.references.length})</span>
          </h3>
          <ul className="space-y-1.5">
            {guidance.references.map((ref, idx) => (
              <li key={idx} className="text-xs leading-relaxed">
                <div className="flex items-start gap-1.5">
                  <span
                    className="px-1 py-0.5 text-[9px] rounded bg-muted text-muted-foreground flex-shrink-0 mt-0.5"
                  >
                    {ref.type === "academic"
                      ? t("guidance.academic")
                      : t("guidance.transparencyResource")}
                  </span>
                  <div className="min-w-0">
                    {ref.url ? (
                      <a
                        href={ref.url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="text-primary hover:underline inline-flex items-center gap-0.5"
                      >
                        {ref.title}
                        <ExternalLink className="h-2.5 w-2.5 flex-shrink-0" />
                      </a>
                    ) : (
                      <span>{ref.title}</span>
                    )}
                    {(ref.authors || ref.year) && (
                      <div className="text-muted-foreground text-[10px]">
                        {[ref.authors, ref.year].filter(Boolean).join(", ")}
                      </div>
                    )}
                  </div>
                </div>
              </li>
            ))}
          </ul>
        </div>
      )}
    </>
  );
}
