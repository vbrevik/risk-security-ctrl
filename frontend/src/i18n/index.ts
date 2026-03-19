import i18n from "i18next";
import { initReactI18next } from "react-i18next";
import LanguageDetector from "i18next-browser-languagedetector";

import enCommon from "./locales/en/common.json";
import enOntology from "./locales/en/ontology.json";
import enCompliance from "./locales/en/compliance.json";
import enReports from "./locales/en/reports.json";
import enAnalysis from "./locales/en/analysis.json";

import nbCommon from "./locales/nb/common.json";
import nbOntology from "./locales/nb/ontology.json";
import nbCompliance from "./locales/nb/compliance.json";
import nbReports from "./locales/nb/reports.json";
import nbAnalysis from "./locales/nb/analysis.json";

const resources = {
  en: {
    common: enCommon,
    ontology: enOntology,
    compliance: enCompliance,
    reports: enReports,
    analysis: enAnalysis,
  },
  nb: {
    common: nbCommon,
    ontology: nbOntology,
    compliance: nbCompliance,
    reports: nbReports,
    analysis: nbAnalysis,
  },
};

i18n
  .use(LanguageDetector)
  .use(initReactI18next)
  .init({
    resources,
    fallbackLng: "en",
    defaultNS: "common",
    interpolation: {
      escapeValue: false,
    },
    detection: {
      order: ["localStorage", "navigator"],
      caches: ["localStorage"],
    },
  });

export default i18n;
