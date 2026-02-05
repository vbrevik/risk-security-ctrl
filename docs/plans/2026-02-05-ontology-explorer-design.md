# Ontology Explorer Design

## Overview

Full-featured ontology visualization for governmental IT security risk management frameworks (ISO 31000, ISO 31010, NIST CSF, ISO 27000). Supports browsing, learning, finding cross-framework connections, and compliance mapping.

## Page Layout

Fits within existing app layout (header/nav provided by root). Content area structure:

```
┌────────────────┬────────────────────────────────────────────────┐
│                │  View Toggle: [Graph] [Detail] [Compare]       │
│   SIDEBAR      │  Toolbar: [🔍 Search] [⌨ Help] [💾 Save] [📷] │
│   (collapsible)│────────────────────────────────────────────────│
│                │                                                │
│  🔍 Filter...  │            MAIN CONTENT AREA                   │
│                │                                                │
│  ▼ ISO 31000   │     (Graph / Detail / Compare view)            │
│    ▼ Principles│                                                │
│      • P1...   │                                                │
│    ► Framework │                                                │
│  ► ISO 31010   │                                                │
│  ► NIST CSF    │                                                │
│                │                                                │
│  🔎 Search all │                                                │
│     [Search]   │                                                │
├────────────────┴────────────────────────────────────────────────┤
│  Status: Selected concept | Connections count | Zoom level      │
└─────────────────────────────────────────────────────────────────┘
```

## Sidebar Component

### Structure
- Collapse toggle button with header
- Quick filter input (top) - client-side fuzzy match on visible tree
- Framework tree - expandable nodes grouped by framework
- Full search input (bottom) - server-side search via API

### Tree Behavior
- Frameworks fetched from `/api/ontology/frameworks`
- Concepts fetched per framework from `/api/ontology/concepts?framework_id=X`
- Tree built from `parent_id` hierarchy
- Click node → select, update main view
- Double-click → expand/collapse
- Concept type icons: category (folder), principle (◆), process (→), technique (⚙)
- Selected node highlighted, synced with graph selection

## Graph View (D3.js)

### Visual Design
- **Nodes**: Rounded rectangles with concept name, colored border by framework
- **Edges**: Curved lines with arrows, labeled with relationship type
- **Layout**: Force-directed with framework clustering
- **Colors**: Framework-specific (ISO 31000=blue, ISO 31010=green, NIST CSF=orange)

### Node States
| State | Appearance |
|-------|------------|
| Default | Framework-colored border, white fill |
| Hover | Elevated shadow, connected nodes/edges highlighted |
| Selected | Filled with framework color, detail panel opens |
| Dimmed | Unrelated nodes fade when one is selected |

### Controls Overlay (bottom-right)
```
[+] [-] [⟲]   ← Zoom in, zoom out, reset view
[🎯] [📍]     ← Fit to screen, toggle minimap
```

### Minimap (top-right)
- Small overview of entire graph
- Viewport rectangle shows current view
- Click to navigate

## Graph Interactions

### Mouse
| Action | Behavior |
|--------|----------|
| Click + drag canvas | Pan |
| Scroll wheel | Zoom |
| Click node | Select, show detail, dim unrelated |
| Ctrl/Cmd + click | Multi-select |
| Click + drag node | Reposition (saved) |
| Double-click node | Expand/collapse relationships |
| Hover node | Tooltip, highlight connected edges |

### Keyboard
| Key | Action |
|-----|--------|
| Arrow keys | Navigate between nodes |
| Enter | Expand/collapse selected |
| Escape | Clear selection |
| Ctrl+F | Focus search |
| Ctrl+S | Save layout |
| Ctrl+E | Export image |
| +/- | Zoom in/out |
| 0 | Reset zoom to fit |
| Tab | Cycle through connected nodes |

### Touch (tablet)
- Pinch to zoom
- Two-finger pan
- Tap to select
- Long press for context menu

## Detail View

Shows comprehensive concept information when selected.

```
┌─────────────────────────────────────────────────────┐
│ [◂ Back to Graph]                    [EN | NB]     │
├─────────────────────────────────────────────────────┤
│ P1 · Integrated                      ISO 31000     │
│ principle                                          │
├─────────────────────────────────────────────────────┤
│ Definition                                         │
│ Risk management is an integral part of all         │
│ organizational activities.                         │
├─────────────────────────────────────────────────────┤
│ Source: ISO 31000:2018 Clause 4a                   │
├─────────────────────────────────────────────────────┤
│ Relationships                              [View ↗]│
│ → implements    NIST.ID-GV-1                       │
│ → related_to    ISO27000.governance                │
│ ← parent_of     P1.1 Integration activities        │
├─────────────────────────────────────────────────────┤
│ Hierarchy                                          │
│ ISO 31000 › Principles › P1 Integrated            │
└─────────────────────────────────────────────────────┘
```

### Features
- Language toggle (EN/NB) using i18n
- Clickable relationships → navigate to concept
- "View ↗" button → graph centered on concept
- Breadcrumb hierarchy → click to navigate up

## Compare View

Side-by-side framework comparison with cross-framework relationships.

```
┌────────────────────────────────────────────────────────────────┐
│ Compare Frameworks                                             │
│ [ISO 31000 ▼]  ←──────────────→  [NIST CSF ▼]                 │
├─────────────────────┬──────────┬───────────────────────────────┤
│   ISO 31000         │ Mappings │      NIST CSF                 │
│   (tree view)       │          │      (tree view)              │
│                     │          │                               │
│ ▼ Principles        │    ═══   │ ▼ Identify (ID)               │
│   ● P1 Integrated ──┼────●────┼──→ ID.GV-1                    │
│   ○ P2 Structured   │          │   ○ ID.GV-2                   │
│                     │          │                               │
│ ▼ Process           │    ═══   │ ▼ Protect (PR)                │
│   ○ Risk Assessment─┼────●────┼──→ PR.IP-7                    │
└─────────────────────┴──────────┴───────────────────────────────┘
```

### Features
- Two dropdown selectors for frameworks
- Side-by-side tree views
- Center column shows relationship lines
- Hover concept → highlight cross-framework mappings
- Click mapping line → relationship details tooltip
- Optional synced scrolling toggle

## Save/Load & Export

### Save Layout (localStorage)
- Graph node positions saved per session
- Data: `{ nodeId: { x, y }, zoom, pan, expandedNodes[] }`
- Auto-saves on change (debounced 500ms)
- Key: `ontology-graph-layout-{frameworkId}`

### Load Layout
- On mount, check localStorage for saved positions
- Apply positions before force simulation
- Fallback to force-directed if no saved state

### Export Dialog
```
┌─────────────────────────────┐
│ Export Graph                │
├─────────────────────────────┤
│ Format:  ○ PNG  ○ SVG       │
│ Size:    ○ Current ○ Full   │
│ Include: ☑ Legend           │
│          ☑ Title            │
│          ☐ Watermark        │
├─────────────────────────────┤
│     [Cancel]  [Export]      │
└─────────────────────────────┘
```

- PNG via `html2canvas`
- SVG via element serialization
- Full size exports entire graph, not just viewport

## File Structure

```
frontend/src/features/ontology/
├── api/
│   └── index.ts              # TanStack Query hooks
├── components/
│   ├── OntologyExplorer.tsx  # Main container
│   ├── Sidebar/
│   │   ├── Sidebar.tsx       # Collapsible wrapper
│   │   ├── FrameworkTree.tsx # Tree component
│   │   ├── TreeNode.tsx      # Recursive node
│   │   └── SearchBox.tsx     # Server-side search
│   ├── Graph/
│   │   ├── GraphView.tsx     # D3 container
│   │   ├── useD3Graph.ts     # D3 logic hook
│   │   ├── GraphControls.tsx # Zoom/pan buttons
│   │   ├── Minimap.tsx       # Overview minimap
│   │   └── GraphNode.tsx     # Node renderer
│   ├── Detail/
│   │   └── DetailView.tsx    # Concept detail panel
│   ├── Compare/
│   │   └── CompareView.tsx   # Side-by-side comparison
│   └── ExportDialog.tsx      # Export modal
├── hooks/
│   ├── useGraphLayout.ts     # Save/load positions
│   ├── useKeyboardNav.ts     # Keyboard shortcuts
│   └── useGraphSelection.ts  # Selection state
├── types/
│   └── index.ts              # TypeScript types
└── utils/
    ├── treeBuilder.ts        # Build tree from flat concepts
    └── graphTransform.ts     # Transform API data → D3 format
```

## API Integration

### TanStack Query Hooks
```typescript
useFrameworks()              → GET /api/ontology/frameworks
useFramework(id)             → GET /api/ontology/frameworks/{id}
useConcepts(frameworkId)     → GET /api/ontology/concepts?framework_id=X
useConcept(id)               → GET /api/ontology/concepts/{id}
useConceptRelationships(id)  → GET /api/ontology/concepts/{id}/relationships
useSearchConcepts(query)     → GET /api/ontology/concepts/search?q=X
useRelationships()           → GET /api/ontology/relationships
```

### Data Flow
1. On mount: Fetch all frameworks → populate sidebar headers
2. On framework expand: Fetch concepts → build tree
3. On concept select: Fetch relationships → update graph edges
4. On search: Server-side search → display results
5. Graph expands incrementally as user explores

### Caching Strategy
| Data | staleTime |
|------|-----------|
| Frameworks | Infinity |
| Concepts | 5 minutes |
| Relationships | 5 minutes |
| Search | 30 seconds |

## Dependencies

Add to `package.json`:
```json
{
  "d3": "^7.x",
  "html2canvas": "^1.x"
}
```

## State Management

| Type | Solution |
|------|----------|
| Server state | TanStack Query |
| UI state | React context (selection, view mode, sidebar) |
| Graph state | D3 internal, positions synced to localStorage |
