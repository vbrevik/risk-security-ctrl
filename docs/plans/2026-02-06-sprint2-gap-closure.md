# Sprint 2 Gap Closure Plan

## Goal
Close all remaining Sprint 2 (Ontology Explorer UI) gaps to reach MVP Ontology Explorer milestone.

## Tasks

### 1. Wire up i18n translations (T2.1 fix)
Replace all hardcoded English strings with `useTranslation()` calls. Add any missing keys to both `en/ontology.json` and `nb/ontology.json`.

**Files to modify:**
- `components/OntologyExplorer.tsx` - view mode labels, keyboard help, status bar
- `components/Detail/DetailView.tsx` - all headings and placeholders
- `components/Sidebar/Sidebar.tsx` - tooltips, placeholder
- `components/Sidebar/FrameworkTree.tsx` - empty states
- `components/Compare/CompareView.tsx` - labels
- `components/ExportDialog.tsx` - all labels
- `i18n/locales/en/ontology.json` - add missing keys
- `i18n/locales/nb/ontology.json` - add missing keys

### 2. Graph: load all frameworks dynamically (T2.2 fix)
Replace hardcoded 3-framework fetch with dynamic loading based on `useFrameworks()`. Remove the `!parent_id` filter and increase the node cap to allow deeper exploration.

**Files to modify:**
- `components/Graph/GraphView.tsx` - dynamic framework loading, remove parent_id filter

### 3. Add debounced search with graph highlighting (T2.4 fix)
Add proper debounce to search input. When in graph view, highlight matching nodes instead of switching to detail view.

**Files to modify:**
- `components/Sidebar/SearchBox.tsx` - add debounce, option to highlight in graph
- `hooks/useD3Graph.ts` - accept highlightedNodeIds prop, visually emphasize them
- `context/ExplorerContext.tsx` - add searchHighlightIds to state

### 4. Add framework & type filters (T2.4 fix)
Add checkboxes for framework multi-select and concept type dropdown in the sidebar.

**Files to modify:**
- `components/Sidebar/Sidebar.tsx` - add filter UI section
- `context/ExplorerContext.tsx` - add filter state (activeFrameworks, activeTypes)
- `components/Graph/GraphView.tsx` - filter nodes based on active filters

### 5. URL parameter persistence (T2.4 fix)
Use TanStack Router search params to persist view mode, selected concept, active filters.

**Files to modify:**
- `routes/ontology/index.tsx` - define search params schema
- `components/OntologyExplorer.tsx` - sync state with URL
- `context/ExplorerContext.tsx` - read initial state from URL params

### 6. Add tree as main-area view mode (T2.5 fix)
Add a "Tree" view mode alongside Graph/Detail/Compare that renders a full hierarchical tree in the main content area.

**Files to modify:**
- `types/index.ts` - add "tree" to ViewMode
- `components/Tree/TreeView.tsx` - new main-area tree component
- `components/OntologyExplorer.tsx` - add tree to view modes, render TreeView

### 7. Update TASKS.md
Mark completed Sprint 1 tasks, update Sprint 2 task checkboxes.

## Execution Order
1 → 2 → 6 → 3 → 4 → 5 → 7

Tasks 1, 2, and 6 are independent and can run in parallel.
Tasks 3 and 4 depend on context changes.
Task 5 depends on all filter state being finalized.
Task 7 is last.
