import { ArrowLeft, ExternalLink, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useConceptRelationships, useFramework } from "../../api";
import { useExplorer } from "../../context";
import { findNodePath, buildTree } from "../../utils";
import { useConcepts } from "../../api";
import { getFrameworkColor } from "../../utils/graphTransform";

export function DetailView() {
  const { t, i18n } = useTranslation("ontology");
  const { state, selectConcept, setViewMode } = useExplorer();
  const selectedId = state.selectedConceptId;

  const { data: conceptData, isLoading } = useConceptRelationships(selectedId ?? "");
  const { data: framework } = useFramework(conceptData?.framework_id ?? "");
  const { data: frameworkConcepts } = useConcepts(conceptData?.framework_id);

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  // Build breadcrumb path
  const breadcrumbPath = (() => {
    if (!frameworkConcepts || !selectedId) return [];
    const tree = buildTree(frameworkConcepts, language);
    return findNodePath(tree, selectedId);
  })();

  if (!selectedId) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Select a concept to view details
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Loading...
      </div>
    );
  }

  if (!conceptData) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Concept not found
      </div>
    );
  }

  const name = language === "nb" && conceptData.name_nb ? conceptData.name_nb : conceptData.name_en;
  const definition = language === "nb" && conceptData.definition_nb
    ? conceptData.definition_nb
    : conceptData.definition_en;

  const borderColor = getFrameworkColor(conceptData.framework_id);

  const handleRelationshipClick = (conceptId: string) => {
    selectConcept(conceptId);
  };

  const handleViewInGraph = () => {
    setViewMode("graph");
  };

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="max-w-2xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setViewMode("graph")}
          >
            <ArrowLeft className="h-4 w-4 mr-2" />
            Back to Graph
          </Button>
          <div className="flex gap-2">
            <Button
              variant={language === "en" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => i18n.changeLanguage("en")}
            >
              EN
            </Button>
            <Button
              variant={language === "nb" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => i18n.changeLanguage("nb")}
            >
              NB
            </Button>
          </div>
        </div>

        {/* Main card */}
        <Card className="border-l-4" style={{ borderLeftColor: borderColor }}>
          <CardHeader>
            <div className="flex items-start justify-between">
              <div>
                <CardTitle className="text-xl">
                  {conceptData.code && (
                    <span className="text-muted-foreground mr-2">{conceptData.code}</span>
                  )}
                  {name}
                </CardTitle>
                <p className="text-sm text-muted-foreground mt-1">
                  {conceptData.concept_type}
                </p>
              </div>
              <span
                className="px-2 py-1 text-xs rounded-full text-white"
                style={{ backgroundColor: borderColor }}
              >
                {framework?.name ?? conceptData.framework_id}
              </span>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Definition */}
            {definition && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Definition
                </h3>
                <p className="text-sm leading-relaxed">{definition}</p>
              </div>
            )}

            {/* Source */}
            {conceptData.source_reference && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Source
                </h3>
                <p className="text-sm">{conceptData.source_reference}</p>
              </div>
            )}

            {/* Relationships */}
            {conceptData.related_concepts.length > 0 && (
              <div>
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-medium text-sm text-muted-foreground">
                    {t("concepts.relationships")}
                  </h3>
                  <Button variant="ghost" size="sm" onClick={handleViewInGraph}>
                    View <ExternalLink className="h-3 w-3 ml-1" />
                  </Button>
                </div>
                <div className="space-y-1">
                  {conceptData.related_concepts.map((rel) => (
                    <button
                      key={rel.relationship_id}
                      onClick={() => handleRelationshipClick(rel.concept_id)}
                      className={cn(
                        "w-full flex items-center gap-2 p-2 text-sm rounded",
                        "hover:bg-accent transition-colors text-left"
                      )}
                    >
                      <span className="text-muted-foreground">
                        {rel.direction === "outgoing" ? "→" : "←"}
                      </span>
                      <span className="text-muted-foreground text-xs">
                        {rel.relationship_type}
                      </span>
                      <span className="flex-1">
                        {language === "nb" && rel.concept_name_nb
                          ? rel.concept_name_nb
                          : rel.concept_name_en}
                      </span>
                    </button>
                  ))}
                </div>
              </div>
            )}

            {/* Breadcrumb */}
            {breadcrumbPath.length > 0 && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Hierarchy
                </h3>
                <div className="flex items-center flex-wrap gap-1 text-sm">
                  <span className="text-muted-foreground">
                    {framework?.name ?? conceptData.framework_id}
                  </span>
                  {breadcrumbPath.map((node, index) => (
                    <span key={node.id} className="flex items-center">
                      <ChevronRight className="h-3 w-3 mx-1 text-muted-foreground" />
                      {index === breadcrumbPath.length - 1 ? (
                        <span className="font-medium">{node.name}</span>
                      ) : (
                        <button
                          onClick={() => selectConcept(node.id)}
                          className="hover:underline"
                        >
                          {node.name}
                        </button>
                      )}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
