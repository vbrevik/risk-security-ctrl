diff --git a/frontend/src/i18n/__tests__/analysis-namespace.test.ts b/frontend/src/i18n/__tests__/analysis-namespace.test.ts
index 3c2a8bd..a7e704f 100644
--- a/frontend/src/i18n/__tests__/analysis-namespace.test.ts
+++ b/frontend/src/i18n/__tests__/analysis-namespace.test.ts
@@ -19,4 +19,48 @@ describe("analysis i18n namespace", () => {
     expect(bundle).toBeDefined();
     expect(bundle.title).toBeTruthy();
   });
+
+  describe("enhancement keys", () => {
+    const radarKeys = [
+      "charts.radar.title",
+      "charts.radar.description",
+      "charts.radar.noData",
+      "charts.radar.addressed",
+      "charts.radar.partial",
+      "charts.radar.gap",
+      "charts.radar.notApplicable",
+      "charts.radar.legend",
+      "charts.radar.percentage",
+    ];
+
+    const conceptPanelKeys = [
+      "detail.conceptPanel.title",
+      "detail.conceptPanel.close",
+      "detail.conceptPanel.openInExplorer",
+      "detail.conceptPanel.definition",
+      "detail.conceptPanel.type",
+      "detail.conceptPanel.framework",
+      "detail.conceptPanel.relatedConcepts",
+      "detail.conceptPanel.crossMappings",
+      "detail.conceptPanel.loading",
+      "detail.conceptPanel.error",
+      "detail.conceptPanel.retry",
+    ];
+
+    const crossFilterKeys = [
+      "detail.filteredBy",
+      "detail.clearFilter",
+    ];
+
+    const allNewKeys = [...radarKeys, ...conceptPanelKeys, ...crossFilterKeys];
+
+    it.each(allNewKeys)("key '%s' exists in en and nb and is non-empty", (key) => {
+      const en = i18n.t(key, { ns: "analysis", lng: "en" });
+      const nb = i18n.t(key, { ns: "analysis", lng: "nb" });
+      expect(en).not.toBe(key);
+      expect(nb).not.toBe(key);
+      expect(en.length).toBeGreaterThan(0);
+      expect(nb.length).toBeGreaterThan(0);
+    });
+  });
 });
diff --git a/frontend/src/i18n/locales/en/analysis.json b/frontend/src/i18n/locales/en/analysis.json
index 614ba9e..7ce1565 100644
--- a/frontend/src/i18n/locales/en/analysis.json
+++ b/frontend/src/i18n/locales/en/analysis.json
@@ -80,7 +80,22 @@
     "notFound": {
       "title": "Analysis not found",
       "message": "The analysis you are looking for does not exist or has been removed."
-    }
+    },
+    "conceptPanel": {
+      "title": "Concept Details",
+      "close": "Close panel",
+      "openInExplorer": "Open in Ontology Explorer",
+      "definition": "Definition",
+      "type": "Type",
+      "framework": "Framework",
+      "relatedConcepts": "Related Concepts",
+      "crossMappings": "Cross-Framework Mappings",
+      "loading": "Loading concept...",
+      "error": "Failed to load concept details",
+      "retry": "Retry"
+    },
+    "filteredBy": "Showing results for: {{framework}}",
+    "clearFilter": "Clear filter"
   },
   "stats": {
     "totalFindings": "Total Findings",
@@ -100,6 +115,17 @@
       "title": "Priority Breakdown",
       "description": "Distribution of findings by priority level",
       "noData": "No priority data available"
+    },
+    "radar": {
+      "title": "Framework Radar",
+      "description": "Normalized finding type distribution across frameworks",
+      "noData": "No radar data available",
+      "addressed": "Addressed",
+      "partial": "Partial",
+      "gap": "Gap",
+      "notApplicable": "Not Applicable",
+      "legend": "Legend",
+      "percentage": "{{value}}%"
     }
   },
   "findings": {
diff --git a/frontend/src/i18n/locales/nb/analysis.json b/frontend/src/i18n/locales/nb/analysis.json
index eef2c45..749685b 100644
--- a/frontend/src/i18n/locales/nb/analysis.json
+++ b/frontend/src/i18n/locales/nb/analysis.json
@@ -80,7 +80,22 @@
     "notFound": {
       "title": "Analyse ikke funnet",
       "message": "Analysen du leter etter finnes ikke eller er fjernet."
-    }
+    },
+    "conceptPanel": {
+      "title": "Konseptdetaljer",
+      "close": "Lukk panel",
+      "openInExplorer": "Åpne i Ontologiutforsker",
+      "definition": "Definisjon",
+      "type": "Type",
+      "framework": "Rammeverk",
+      "relatedConcepts": "Relaterte konsepter",
+      "crossMappings": "Kryssrammeverk-koblinger",
+      "loading": "Laster konsept...",
+      "error": "Kunne ikke laste konseptdetaljer",
+      "retry": "Prøv igjen"
+    },
+    "filteredBy": "Viser resultater for: {{framework}}",
+    "clearFilter": "Fjern filter"
   },
   "stats": {
     "totalFindings": "Totalt antall funn",
@@ -100,6 +115,17 @@
       "title": "Prioritetsfordeling",
       "description": "Fordeling av funn etter prioritetsnivå",
       "noData": "Ingen prioritetsdata tilgjengelig"
+    },
+    "radar": {
+      "title": "Rammeverkradar",
+      "description": "Normalisert funntype-fordeling på tvers av rammeverk",
+      "noData": "Ingen radardata tilgjengelig",
+      "addressed": "Adressert",
+      "partial": "Delvis",
+      "gap": "Mangel",
+      "notApplicable": "Ikke relevant",
+      "legend": "Tegnforklaring",
+      "percentage": "{{value}}%"
     }
   },
   "findings": {
