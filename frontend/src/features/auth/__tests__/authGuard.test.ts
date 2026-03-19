import { describe, it, expect } from "vitest";
import { requireAuth } from "../authGuard";

describe("requireAuth", () => {
  it("does not throw when isAuthenticated is true", () => {
    expect(() => {
      requireAuth({ auth: { isAuthenticated: true } });
    }).not.toThrow();
  });

  it("throws redirect when isAuthenticated is false", () => {
    try {
      requireAuth({ auth: { isAuthenticated: false } });
      expect.fail("Expected requireAuth to throw");
    } catch (error: unknown) {
      // TanStack Router redirect returns an object with options.to
      const redirectObj = error as { options?: { to?: string } };
      expect(redirectObj.options?.to).toBe("/login");
    }
  });

  it("thrown redirect includes search.redirect param", () => {
    try {
      requireAuth({ auth: { isAuthenticated: false } });
      expect.fail("Expected requireAuth to throw");
    } catch (error: unknown) {
      const redirectObj = error as {
        options?: { search?: { redirect?: string } };
      };
      expect(redirectObj.options?.search).toBeDefined();
      expect(redirectObj.options?.search?.redirect).toBeDefined();
    }
  });
});
