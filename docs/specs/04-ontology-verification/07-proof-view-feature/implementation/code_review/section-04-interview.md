# Code Review Interview: section-04-proof-panel

## Moderate: Weak error test assertion — auto-fix applied
Changed `expect(container.textContent).toBeTruthy()` to `expect(screen.getByText("Could not load proof document.")).toBeInTheDocument()`. Now directly asserts the safe generic message text.

## Moderate: i18n fallback strings — per-spec decision, not fixed
The t(key, fallback) pattern with English fallbacks is intentional per the section-04 spec: "the component will still compile without [keys] — keys will fall back to the key string." Section-06 adds all proof.* keys to both locale files. The fallbacks serve as development scaffolding, not production behavior.

## Minor: useMemo on ReactMarkdown — kept per spec
The plan explicitly requires `useMemo` keyed on `proof_content`. Keeping it despite limited practical benefit since it's a spec requirement and has no correctness impact.
