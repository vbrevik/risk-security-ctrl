import { describe, it, expect } from "vitest";
import i18n from "../index";

describe("analysis i18n namespace", () => {
  it("analysis namespace is registered and loadable", () => {
    const bundle = i18n.getResourceBundle("en", "analysis");
    expect(bundle).toBeDefined();
    expect(bundle.title).toBeTruthy();
  });

  it("key access returns translated string, not the key itself", () => {
    const result = i18n.t("title", { ns: "analysis", lng: "en" });
    expect(result).not.toBe("title");
    expect(result).toBe("Document Analysis");
  });

  it("nb locale has analysis namespace", () => {
    const bundle = i18n.getResourceBundle("nb", "analysis");
    expect(bundle).toBeDefined();
    expect(bundle.title).toBeTruthy();
  });

  describe("enhancement keys", () => {
    const radarKeys = [
      "charts.radar.title",
      "charts.radar.description",
      "charts.radar.noData",
      "charts.radar.addressed",
      "charts.radar.partial",
      "charts.radar.gap",
      "charts.radar.notApplicable",
      "charts.radar.legend",
      "charts.radar.percentage",
    ];

    const conceptPanelKeys = [
      "detail.conceptPanel.title",
      "detail.conceptPanel.close",
      "detail.conceptPanel.openInExplorer",
      "detail.conceptPanel.definition",
      "detail.conceptPanel.type",
      "detail.conceptPanel.framework",
      "detail.conceptPanel.relatedConcepts",
      "detail.conceptPanel.crossMappings",
      "detail.conceptPanel.loading",
      "detail.conceptPanel.error",
      "detail.conceptPanel.retry",
    ];

    const crossFilterKeys = [
      "detail.filteredBy",
      "detail.clearFilter",
    ];

    const allNewKeys = [...radarKeys, ...conceptPanelKeys, ...crossFilterKeys];

    it.each(allNewKeys)("key '%s' exists in en and nb and is non-empty", (key) => {
      const en = i18n.t(key, { ns: "analysis", lng: "en" });
      const nb = i18n.t(key, { ns: "analysis", lng: "nb" });
      expect(en).not.toBe(key);
      expect(nb).not.toBe(key);
      expect(en.length).toBeGreaterThan(0);
      expect(nb.length).toBeGreaterThan(0);
    });
  });
});
