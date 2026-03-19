# Frontend Auth — Interview Transcript

## Q1: UI component approach?
**Answer:** shadcn/ui components (Input, Button, Card, Label). Consistent with rest of app.

## Q2: Auth loading state UX?
**Answer:** Full-screen spinner while checking /api/auth/me on initial load.

## Q3: Session expiry UX?
**Answer:** Redirect to /login with toast message "Session expired, please log in again."

## Prior Decisions (from deep-project interview)
- Public pages: Home only
- Token handling: Cookie-based (automatic) + Bearer header for API clients
- Registration: Open, anyone can sign up as viewer
- i18n: Add `auth` namespace for en + nb
- CSRF: Frontend must send X-Requested-With: XMLHttpRequest on all mutations
