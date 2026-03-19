import { useTranslation } from "react-i18next";
import { ExternalLink } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Sheet,
  SheetContent,
  SheetHeader,
  SheetTitle,
  SheetDescription,
} from "@/components/ui/sheet";
import { useConceptRelationships } from "@/features/ontology/api";

interface ConceptDrawerProps {
  conceptId: string | null;
  onClose: () => void;
}

export function ConceptDrawer({ conceptId, onClose }: ConceptDrawerProps) {
  const { t } = useTranslation("analysis");
  const { data, isLoading, isError, refetch } = useConceptRelationships(
    conceptId ?? ""
  );

  return (
    <Sheet open={conceptId !== null} onOpenChange={(open) => !open && onClose()}>
      <SheetContent side="right" className="w-[400px] overflow-y-auto">
        <SheetHeader>
          <SheetTitle>{t("detail.conceptPanel.title")}</SheetTitle>
          {data?.code && (
            <SheetDescription className="font-mono">
              {data.code}
            </SheetDescription>
          )}
        </SheetHeader>

        {isLoading && (
          <div
            className="space-y-4 mt-4"
            aria-label={t("detail.conceptPanel.loading")}
          >
            <div className="h-6 w-3/4 bg-muted rounded animate-pulse" />
            <div className="h-4 w-1/2 bg-muted rounded animate-pulse" />
            <div className="h-20 w-full bg-muted rounded animate-pulse" />
          </div>
        )}

        {isError && (
          <div className="mt-4 space-y-3">
            <p className="text-sm text-destructive">
              {t("detail.conceptPanel.error")}
            </p>
            <Button variant="outline" size="sm" onClick={() => refetch()}>
              {t("detail.conceptPanel.retry")}
            </Button>
          </div>
        )}

        {data && !isLoading && !isError && (
          <div className="space-y-5 mt-4">
            {/* Concept name */}
            <h3 className="text-lg font-semibold">{data.name_en}</h3>

            {/* Metadata */}
            <div className="grid grid-cols-2 gap-2 text-sm">
              <div>
                <p className="text-muted-foreground">
                  {t("detail.conceptPanel.type")}
                </p>
                <p className="font-medium">{data.concept_type}</p>
              </div>
              <div>
                <p className="text-muted-foreground">
                  {t("detail.conceptPanel.framework")}
                </p>
                <p className="font-medium">{data.framework_id}</p>
              </div>
            </div>

            {/* Definition */}
            {data.definition_en && (
              <div>
                <p className="text-sm font-semibold mb-1">
                  {t("detail.conceptPanel.definition")}
                </p>
                <p className="text-sm text-muted-foreground">
                  {data.definition_en}
                </p>
              </div>
            )}

            {/* Related concepts */}
            {data.related_concepts.length > 0 && (
              <div>
                <p className="text-sm font-semibold mb-2">
                  {t("detail.conceptPanel.relatedConcepts")}
                </p>
                <div className="space-y-1">
                  {data.related_concepts
                    .filter(
                      (rc) => rc.concept_framework_id === data.framework_id
                    )
                    .map((rc) => (
                      <div
                        key={rc.relationship_id}
                        className="text-xs flex items-center gap-2"
                      >
                        <span className="text-muted-foreground">
                          {rc.relationship_type}
                        </span>
                        <span>{rc.concept_name_en}</span>
                      </div>
                    ))}
                </div>
              </div>
            )}

            {/* Cross-framework mappings */}
            {data.related_concepts.some(
              (rc) => rc.concept_framework_id !== data.framework_id
            ) && (
              <div>
                <p className="text-sm font-semibold mb-2">
                  {t("detail.conceptPanel.crossMappings")}
                </p>
                <div className="space-y-1">
                  {data.related_concepts
                    .filter(
                      (rc) => rc.concept_framework_id !== data.framework_id
                    )
                    .map((rc) => (
                      <div
                        key={rc.relationship_id}
                        className="text-xs flex items-center gap-2"
                      >
                        <span className="text-muted-foreground font-mono">
                          {rc.concept_framework_id}
                        </span>
                        <span>{rc.concept_name_en}</span>
                      </div>
                    ))}
                </div>
              </div>
            )}

            {/* Open in explorer link */}
            <a
              href={`/ontology?concept=${conceptId}`}
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 text-sm text-accent-foreground hover:underline mt-4"
            >
              <ExternalLink className="h-3.5 w-3.5" />
              {t("detail.conceptPanel.openInExplorer")}
            </a>
          </div>
        )}
      </SheetContent>
    </Sheet>
  );
}
