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
});
