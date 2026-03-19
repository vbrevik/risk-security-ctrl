Now I have everything I need. Here is the section content:

# Section 7: i18n Translations

## Overview

This section creates the `auth` i18n namespace with English and Norwegian Bokmal translation files, and registers the namespace in the i18next configuration. This section has **no dependencies** on other sections and can be implemented in parallel with sections 01 through 06. Sections 04 (Login Page) and 05 (Register Page) depend on this section being complete so they can use `useTranslation('auth')`.

## Tests First

Create the test file at `frontend/src/i18n/__tests__/auth-namespace.test.ts`. Follow the pattern established by the existing `analysis-namespace.test.ts` test.

```typescript
// frontend/src/i18n/__tests__/auth-namespace.test.ts
import { describe, it, expect } from "vitest";
import i18n from "../index";

describe("auth i18n namespace", () => {
  it("auth namespace is registered and loadable for en", () => {
    const bundle = i18n.getResourceBundle("en", "auth");
    expect(bundle).toBeDefined();
    expect(bundle.login).toBeDefined();
    expect(bundle.login.title).toBeTruthy();
  });

  it("auth namespace is registered and loadable for nb", () => {
    const bundle = i18n.getResourceBundle("nb", "auth");
    expect(bundle).toBeDefined();
    expect(bundle.login).toBeDefined();
    expect(bundle.login.title).toBeTruthy();
  });

  it("key access returns translated string, not the key itself", () => {
    const result = i18n.t("login.title", { ns: "auth", lng: "en" });
    expect(result).not.toBe("login.title");
    expect(result).toBe("Sign In");
  });

  describe("all auth keys exist in both locales", () => {
    const keys = [
      "login.title",
      "login.email",
      "login.password",
      "login.submit",
      "login.noAccount",
      "login.register",
      "login.error",
      "register.title",
      "register.email",
      "register.name",
      "register.password",
      "register.confirmPassword",
      "register.submit",
      "register.hasAccount",
      "register.login",
      "register.passwordMismatch",
      "register.passwordTooShort",
      "register.success",
      "session.expired",
      "navbar.logout",
    ];

    it.each(keys)("key '%s' exists in en and nb and is non-empty", (key) => {
      const en = i18n.t(key, { ns: "auth", lng: "en" });
      const nb = i18n.t(key, { ns: "auth", lng: "nb" });
      expect(en).not.toBe(key);
      expect(nb).not.toBe(key);
      expect(en.length).toBeGreaterThan(0);
      expect(nb.length).toBeGreaterThan(0);
    });
  });
});
```

Run tests with: `cd /Users/vidarbrevik/projects/risk-security-ctrl/frontend && pnpm test src/i18n/__tests__/auth-namespace.test.ts`

Tests will fail until the translation files and namespace registration are in place.

## Implementation

### Step 1: Create English translation file

Create `frontend/src/i18n/locales/en/auth.json` with the following content:

```json
{
  "login": {
    "title": "Sign In",
    "email": "Email",
    "password": "Password",
    "submit": "Sign In",
    "noAccount": "Don't have an account?",
    "register": "Create Account",
    "error": "Invalid email or password"
  },
  "register": {
    "title": "Create Account",
    "email": "Email",
    "name": "Full Name",
    "password": "Password",
    "confirmPassword": "Confirm Password",
    "submit": "Create Account",
    "hasAccount": "Already have an account?",
    "login": "Sign In",
    "passwordMismatch": "Passwords do not match",
    "passwordTooShort": "Password must be at least 8 characters",
    "success": "Account created. Please sign in."
  },
  "session": {
    "expired": "Session expired. Please sign in again."
  },
  "navbar": {
    "logout": "Sign Out"
  }
}
```

### Step 2: Create Norwegian Bokmal translation file

Create `frontend/src/i18n/locales/nb/auth.json` with Norwegian translations for all the same keys:

```json
{
  "login": {
    "title": "Logg inn",
    "email": "E-post",
    "password": "Passord",
    "submit": "Logg inn",
    "noAccount": "Har du ikke en konto?",
    "register": "Opprett konto",
    "error": "Ugyldig e-post eller passord"
  },
  "register": {
    "title": "Opprett konto",
    "email": "E-post",
    "name": "Fullt navn",
    "password": "Passord",
    "confirmPassword": "Bekreft passord",
    "submit": "Opprett konto",
    "hasAccount": "Har du allerede en konto?",
    "login": "Logg inn",
    "passwordMismatch": "Passordene stemmer ikke overens",
    "passwordTooShort": "Passordet ma vaere minst 8 tegn",
    "success": "Konto opprettet. Vennligst logg inn."
  },
  "session": {
    "expired": "Oekten har utlopt. Vennligst logg inn igjen."
  },
  "navbar": {
    "logout": "Logg ut"
  }
}
```

Note: The Norwegian text above uses ASCII-safe approximations. The implementer should use proper Norwegian characters (a-ring, o-slash, ae-ligature) where appropriate: "ma" should be "ma med ring-a", "vaere" should have ae-ligature, "Oekten" should be "Okten" with o-slash, "utlopt" should have o-slash.

### Step 3: Register the namespace in i18next config

Modify `frontend/src/i18n/index.ts` to import and register the auth namespace. The changes follow the exact same pattern used for every other namespace in the file:

1. Add two import lines (after the existing analysis imports):
   - `import enAuth from "./locales/en/auth.json";`
   - `import nbAuth from "./locales/nb/auth.json";`

2. Add `auth: enAuth` to the `en` object in the `resources` constant.

3. Add `auth: nbAuth` to the `nb` object in the `resources` constant.

The resulting resources object will include `auth` alongside `common`, `ontology`, `compliance`, `reports`, and `analysis`.

## Files Summary

| File | Action |
|------|--------|
| `frontend/src/i18n/__tests__/auth-namespace.test.ts` | Create (test file) |
| `frontend/src/i18n/locales/en/auth.json` | Create |
| `frontend/src/i18n/locales/nb/auth.json` | Create |
| `frontend/src/i18n/index.ts` | Modify (add imports and namespace registration) |

## Usage by Other Sections

Sections 04 (Login Page) and 05 (Register Page) will use these translations via:

```typescript
const { t } = useTranslation('auth');
// Then: t('login.title'), t('register.passwordMismatch'), etc.
```

Section 01 (API Client) uses `t('session.expired', { ns: 'auth' })` in the 401 interceptor toast.

Section 06 (Navbar) uses `t('navbar.logout', { ns: 'auth' })` for the sign-out button label.