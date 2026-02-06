# Ontology Explorer: Connected Navigation via Context Panel

**Date:** 2026-02-06
**Goal:** Replace isolated view modes with a persistent context panel that keeps concept information visible across all views, enabling fluid navigation through the ontology.

## Problem

The Ontology Explorer has four separate view modes (graph, tree, detail, compare) that feel disconnected. Selecting a concept in the graph and switching to tree loses context. The detail view is a full-page replacement that hides the spatial visualization. Compare view is a dead end with no click-through.

## Solution: Persistent Context Panel

A 320px-wide right side panel that appears when any concept is selected, regardless of which view is active. It replaces the full-page Detail view and keeps concept information visible while exploring.

## Architecture

### Context Panel (`ContextPanel.tsx`)

**Placement:** Right side of the main view area, slides in on concept selection.

**Content (top to bottom):**
1. Navigation breadcrumb trail (clickable history of visited concepts)
2. Header: concept code + name, framework color dot, close button
3. Type badge with icon
4. Definition (translated EN/NB)
5. Relationships list (clickable, grouped by direction)
6. Cross-framework mappings (grouped by target framework with color dots)
7. Source reference

**Behavior:**
- Collapsed when no concept selected (main view uses full width)
- Slides in with 200ms transition on selection
- Stays open across view switches (graph -> tree -> compare)
- Close button dismisses and clears selection
- Clicking a relationship updates the panel AND syncs the active view

### Navigation Breadcrumb Trail

Horizontal trail at top of panel showing navigation path:

```
ISO 31000 > Risk Management > Risk Assessment > Risk Identification
```

- Shows navigation path (not hierarchy) — cross-framework jumps appear in sequence
- Click any item to navigate back (truncates forward history)
- Max 5 visible items, earlier collapse behind `...` dropdown
- Resets on panel close

### View Synchronization

All views react to concept selection from any source:

**Graph view:**
- Auto-pans to center selected node (300ms smooth transition)
- If concept not in current graph, shows toast notification
- Existing: bold ring + connected edge highlights

**Tree view:**
- Auto-expands parent chain to selected concept
- Scrolls into view with highlight pulse animation (200ms)
- Selected item gets persistent left-border accent in framework color

**Compare view:**
- Concepts become clickable (opens context panel) — currently not clickable
- Selected concept highlights with left-border accent
- Cross-mapping lines involving selected concept highlight; others dim to 20% opacity

### State Changes

**ExplorerContext additions:**
- `navigationHistory: string[]` — ordered list of visited concept IDs
- `PUSH_NAVIGATION(conceptId)` action — appended on every SELECT_CONCEPT
- `NAVIGATE_BACK(conceptId)` action — truncates history to that point

**ExplorerContext removals:**
- `"detail"` from ViewMode type (3 modes: graph, tree, compare)
- URL param `view=detail` no longer valid (redirects to graph with concept)

### Component Changes

| Component | Change |
|-----------|--------|
| `ContextPanel.tsx` | **New** — persistent right panel |
| `ExplorerContext.tsx` | Add navigation history, remove detail mode |
| `ExplorerContent.tsx` | Add context panel to layout |
| `GraphView.tsx` | Auto-pan to externally selected nodes |
| `TreeView.tsx` | Auto-expand + scroll to selected concept |
| `CompareView.tsx` | Make concepts clickable, highlight selection |
| Toolbar | Remove Detail view button |
| `DetailView.tsx` | **Remove** — replaced by ContextPanel |

### i18n Keys

```json
{
  "panel": {
    "definition": "Definition",
    "relationships": "Relationships",
    "crossMappings": "Cross-framework Mappings",
    "source": "Source",
    "close": "Close",
    "conceptNotInView": "Concept not visible in current view"
  }
}
```

## Implementation Order

1. Add navigation history to ExplorerContext
2. Build ContextPanel component (using DetailView content as starting point)
3. Wire ContextPanel into ExplorerContent layout
4. Remove Detail view mode from toolbar and router
5. Add auto-pan to GraphView
6. Add auto-expand + scroll to TreeView
7. Make CompareView concepts clickable with highlights
8. Add i18n keys (en + nb)
9. Delete DetailView component
10. Update URL param handling
