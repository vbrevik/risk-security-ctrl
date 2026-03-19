import { useState, useEffect } from "react";
import { useTranslation } from "react-i18next";
import { Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Card, CardContent } from "@/components/ui/card";
import { usePromptTemplate, useUpdatePromptTemplate } from "../api";
import { BoostTermsEditor } from "./BoostTermsEditor";
import type { MatcherConfig } from "../types";

const DEFAULT_MATCHER_CONFIG: MatcherConfig = {
  version: 1,
  thresholds: {
    min_confidence: 0.1,
    addressed: 0.6,
    partial: 0.3,
  },
  max_findings_per_framework: 50,
  include_addressed_findings: true,
  boost_terms: {},
};

export function SettingsForm() {
  const { t } = useTranslation("analysis");
  const { data, isLoading, isError } = usePromptTemplate();
  const updateMutation = useUpdatePromptTemplate();

  const [minConfidence, setMinConfidence] = useState(0.1);
  const [addressed, setAddressed] = useState(0.6);
  const [partial, setPartial] = useState(0.3);
  const [maxFindings, setMaxFindings] = useState(50);
  const [includeAddressed, setIncludeAddressed] = useState(true);
  const [boostTerms, setBoostTerms] = useState<Array<{ term: string; weight: number }>>([]);
  const [showSaved, setShowSaved] = useState(false);

  useEffect(() => {
    if (data) {
      setMinConfidence(data.thresholds.min_confidence);
      setAddressed(data.thresholds.addressed);
      setPartial(data.thresholds.partial);
      setMaxFindings(data.max_findings_per_framework);
      setIncludeAddressed(data.include_addressed_findings);
      setBoostTerms(
        Object.entries(data.boost_terms).map(([term, weight]) => ({ term, weight }))
      );
    }
  }, [data]);

  if (isLoading) {
    return (
      <div className="flex justify-center py-12">
        <Loader2 className="h-6 w-6 animate-spin text-muted-foreground" />
      </div>
    );
  }

  if (isError) {
    return (
      <div className="text-center py-12">
        <p className="text-destructive">{t("common.error")}</p>
      </div>
    );
  }

  function handleSave() {
    const config: MatcherConfig = {
      version: data?.version ?? 1,
      thresholds: {
        min_confidence: minConfidence,
        addressed,
        partial,
      },
      max_findings_per_framework: maxFindings,
      include_addressed_findings: includeAddressed,
      boost_terms: Object.fromEntries(
        boostTerms
          .filter((t) => t.term.trim())
          .map((t) => [t.term.trim(), t.weight])
      ),
    };
    updateMutation.mutate(config);
    setShowSaved(true);
    setTimeout(() => setShowSaved(false), 2000);
  }

  function handleReset() {
    if (window.confirm(t("settings.resetConfirm"))) {
      setMinConfidence(DEFAULT_MATCHER_CONFIG.thresholds.min_confidence);
      setAddressed(DEFAULT_MATCHER_CONFIG.thresholds.addressed);
      setPartial(DEFAULT_MATCHER_CONFIG.thresholds.partial);
      setMaxFindings(DEFAULT_MATCHER_CONFIG.max_findings_per_framework);
      setIncludeAddressed(DEFAULT_MATCHER_CONFIG.include_addressed_findings);
      setBoostTerms([]);
    }
  }

  return (
    <Card>
      <CardContent className="space-y-8 pt-6">
        <div className="space-y-4">
          <h3 className="text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            {t("settings.thresholds")}
          </h3>
          <div className="grid gap-4 sm:grid-cols-2">
            <div className="space-y-2">
              <Label>{t("settings.minConfidence")}</Label>
              <Input
                type="number"
                value={minConfidence}
                onChange={(e) => setMinConfidence(parseFloat(e.target.value) || 0)}
                min={0} max={1} step={0.05}
              />
            </div>
            <div className="space-y-2">
              <Label>{t("settings.addressedThreshold")}</Label>
              <Input
                type="number"
                value={addressed}
                onChange={(e) => setAddressed(parseFloat(e.target.value) || 0)}
                min={0} max={1} step={0.05}
              />
            </div>
            <div className="space-y-2">
              <Label>{t("settings.partialThreshold")}</Label>
              <Input
                type="number"
                value={partial}
                onChange={(e) => setPartial(parseFloat(e.target.value) || 0)}
                min={0} max={1} step={0.05}
              />
            </div>
            <div className="space-y-2">
              <Label>{t("settings.maxFindings")}</Label>
              <Input
                type="number"
                value={maxFindings}
                onChange={(e) => setMaxFindings(parseInt(e.target.value) || 0)}
                min={1} max={500} step={1}
              />
            </div>
          </div>
        </div>

        <div className="flex items-center gap-3">
          <input
            type="checkbox"
            id="includeAddressed"
            checked={includeAddressed}
            onChange={(e) => setIncludeAddressed(e.target.checked)}
            className="h-4 w-4 rounded border-border"
          />
          <Label htmlFor="includeAddressed">{t("settings.includeAddressed")}</Label>
        </div>

        <div className="space-y-4">
          <h3 className="text-sm font-semibold uppercase tracking-wider text-muted-foreground">
            {t("settings.boostTerms")}
          </h3>
          <BoostTermsEditor value={boostTerms} onChange={setBoostTerms} />
        </div>

        <div className="flex items-center gap-3">
          <Button onClick={handleSave} disabled={updateMutation.isPending}>
            {updateMutation.isPending ? (
              <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            ) : null}
            {t("settings.save")}
          </Button>
          {showSaved && (
            <span className="text-sm text-green-600">{t("settings.saved")}</span>
          )}
          <Button variant="outline" onClick={handleReset}>
            {t("settings.resetDefaults")}
          </Button>
        </div>
      </CardContent>
    </Card>
  );
}
