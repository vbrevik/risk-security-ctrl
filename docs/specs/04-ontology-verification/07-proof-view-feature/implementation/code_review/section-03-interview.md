# Code Review Interview: section-03-verification-badge

## Moderate: aria-label hardcoded English — auto-fix applied
Changed `aria-label={config.label}` to `aria-label={t(config.i18nKey, config.label)}`. The aria-label now uses the translated value, consistent with the visible span text and the project's i18n requirement.
