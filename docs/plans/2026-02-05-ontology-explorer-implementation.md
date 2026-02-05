# Ontology Explorer Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Build a full-featured ontology visualization with collapsible sidebar tree, D3.js graph, detail view, and framework comparison.

**Architecture:** Feature-based module at `frontend/src/features/ontology/` with TanStack Query for server state, React context for UI state, D3.js for graph rendering. Sidebar tree built from hierarchical concept data, graph uses force-directed layout with framework clustering.

**Tech Stack:** React 19, TypeScript, TanStack Query, D3.js v7, Tailwind CSS v4, i18next, html2canvas

---

## Task 1: Install Dependencies

**Files:**
- Modify: `frontend/package.json`

**Step 1: Add D3.js and html2canvas**

```bash
cd frontend && pnpm add d3 html2canvas && pnpm add -D @types/d3
```

**Step 2: Verify installation**

Run: `cd frontend && pnpm list d3 html2canvas`
Expected: Both packages listed with versions

**Step 3: Commit**

```bash
git add frontend/package.json frontend/pnpm-lock.yaml
git commit -m "chore: add d3 and html2canvas dependencies"
```

---

## Task 2: Create TypeScript Types

**Files:**
- Create: `frontend/src/features/ontology/types/index.ts`

**Step 1: Create types file**

```typescript
// API Response Types (match backend models)
export interface Framework {
  id: string;
  name: string;
  version: string | null;
  description: string | null;
  source_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface Concept {
  id: string;
  framework_id: string;
  parent_id: string | null;
  concept_type: string;
  code: string | null;
  name_en: string;
  name_nb: string | null;
  definition_en: string | null;
  definition_nb: string | null;
  source_reference: string | null;
  sort_order: number | null;
  created_at: string;
  updated_at: string;
}

export interface Relationship {
  id: string;
  source_concept_id: string;
  target_concept_id: string;
  relationship_type: string;
  description: string | null;
  created_at: string | null;
}

export interface RelatedConcept {
  relationship_id: string;
  relationship_type: string;
  concept_id: string;
  concept_name_en: string;
  concept_name_nb: string | null;
  direction: "incoming" | "outgoing";
}

export interface ConceptWithRelationships extends Concept {
  related_concepts: RelatedConcept[];
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  page: number;
  limit: number;
  total_pages: number;
}

// Tree Types
export interface TreeNode {
  id: string;
  name: string;
  type: string;
  code: string | null;
  children: TreeNode[];
  concept: Concept;
  isExpanded: boolean;
}

export interface FrameworkTreeData {
  framework: Framework;
  tree: TreeNode[];
  isExpanded: boolean;
  isLoading: boolean;
}

// Graph Types
export interface GraphNode {
  id: string;
  name: string;
  type: string;
  frameworkId: string;
  code: string | null;
  x?: number;
  y?: number;
  fx?: number | null;
  fy?: number | null;
}

export interface GraphEdge {
  id: string;
  source: string | GraphNode;
  target: string | GraphNode;
  type: string;
  label: string;
}

export interface GraphData {
  nodes: GraphNode[];
  edges: GraphEdge[];
}

// UI State Types
export type ViewMode = "graph" | "detail" | "compare";

export interface ExplorerState {
  selectedConceptId: string | null;
  selectedConcepts: string[];
  viewMode: ViewMode;
  sidebarCollapsed: boolean;
  compareFrameworks: [string | null, string | null];
}

// Layout Persistence Types
export interface NodePosition {
  x: number;
  y: number;
}

export interface SavedLayout {
  positions: Record<string, NodePosition>;
  zoom: number;
  panX: number;
  panY: number;
  expandedNodes: string[];
  timestamp: number;
}
```

**Step 2: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 3: Commit**

```bash
git add frontend/src/features/ontology/types/index.ts
git commit -m "feat(ontology): add TypeScript types for ontology explorer"
```

---

## Task 3: Create API Hooks

**Files:**
- Modify: `frontend/src/features/ontology/api/index.ts`

**Step 1: Write API hooks**

```typescript
import { useQuery } from "@tanstack/react-query";
import { api } from "@/lib/api";
import type {
  Framework,
  Concept,
  Relationship,
  ConceptWithRelationships,
  PaginatedResponse,
} from "../types";

// Query keys
export const ontologyKeys = {
  all: ["ontology"] as const,
  frameworks: () => [...ontologyKeys.all, "frameworks"] as const,
  framework: (id: string) => [...ontologyKeys.frameworks(), id] as const,
  concepts: (frameworkId?: string) =>
    [...ontologyKeys.all, "concepts", { frameworkId }] as const,
  concept: (id: string) => [...ontologyKeys.all, "concept", id] as const,
  conceptRelationships: (id: string) =>
    [...ontologyKeys.concept(id), "relationships"] as const,
  relationships: () => [...ontologyKeys.all, "relationships"] as const,
  search: (query: string, frameworkId?: string) =>
    [...ontologyKeys.all, "search", { query, frameworkId }] as const,
};

// Fetch all frameworks
export function useFrameworks() {
  return useQuery({
    queryKey: ontologyKeys.frameworks(),
    queryFn: async () => {
      const { data } = await api.get<Framework[]>("/ontology/frameworks");
      return data;
    },
    staleTime: Infinity,
  });
}

// Fetch single framework
export function useFramework(id: string) {
  return useQuery({
    queryKey: ontologyKeys.framework(id),
    queryFn: async () => {
      const { data } = await api.get<Framework>(`/ontology/frameworks/${id}`);
      return data;
    },
    staleTime: Infinity,
    enabled: !!id,
  });
}

// Fetch concepts for a framework
export function useConcepts(frameworkId?: string) {
  return useQuery({
    queryKey: ontologyKeys.concepts(frameworkId),
    queryFn: async () => {
      const params = new URLSearchParams();
      if (frameworkId) params.set("framework_id", frameworkId);
      params.set("limit", "500"); // Fetch all concepts for tree building
      const { data } = await api.get<PaginatedResponse<Concept>>(
        `/ontology/concepts?${params}`
      );
      return data.data;
    },
    staleTime: 1000 * 60 * 5, // 5 minutes
    enabled: !!frameworkId,
  });
}

// Fetch single concept
export function useConcept(id: string) {
  return useQuery({
    queryKey: ontologyKeys.concept(id),
    queryFn: async () => {
      const { data } = await api.get<Concept>(`/ontology/concepts/${id}`);
      return data;
    },
    staleTime: 1000 * 60 * 5,
    enabled: !!id,
  });
}

// Fetch concept with relationships
export function useConceptRelationships(id: string) {
  return useQuery({
    queryKey: ontologyKeys.conceptRelationships(id),
    queryFn: async () => {
      const { data } = await api.get<ConceptWithRelationships>(
        `/ontology/concepts/${id}/relationships`
      );
      return data;
    },
    staleTime: 1000 * 60 * 5,
    enabled: !!id,
  });
}

// Fetch all relationships
export function useRelationships() {
  return useQuery({
    queryKey: ontologyKeys.relationships(),
    queryFn: async () => {
      const { data } = await api.get<Relationship[]>("/ontology/relationships");
      return data;
    },
    staleTime: 1000 * 60 * 5,
  });
}

// Search concepts
export function useSearchConcepts(query: string, frameworkId?: string) {
  return useQuery({
    queryKey: ontologyKeys.search(query, frameworkId),
    queryFn: async () => {
      const params = new URLSearchParams({ q: query });
      if (frameworkId) params.set("framework_id", frameworkId);
      const { data } = await api.get<PaginatedResponse<Concept>>(
        `/ontology/concepts/search?${params}`
      );
      return data.data;
    },
    staleTime: 1000 * 30, // 30 seconds
    enabled: query.length >= 2,
  });
}
```

**Step 2: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 3: Commit**

```bash
git add frontend/src/features/ontology/api/index.ts
git commit -m "feat(ontology): add TanStack Query hooks for ontology API"
```

---

## Task 4: Create Utility Functions

**Files:**
- Create: `frontend/src/features/ontology/utils/treeBuilder.ts`
- Create: `frontend/src/features/ontology/utils/graphTransform.ts`
- Create: `frontend/src/features/ontology/utils/index.ts`

**Step 1: Create tree builder utility**

```typescript
// frontend/src/features/ontology/utils/treeBuilder.ts
import type { Concept, TreeNode } from "../types";

export function buildTree(concepts: Concept[], language: "en" | "nb" = "en"): TreeNode[] {
  const nodeMap = new Map<string, TreeNode>();
  const roots: TreeNode[] = [];

  // Create nodes for all concepts
  for (const concept of concepts) {
    const name = language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en;
    nodeMap.set(concept.id, {
      id: concept.id,
      name,
      type: concept.concept_type,
      code: concept.code,
      children: [],
      concept,
      isExpanded: false,
    });
  }

  // Build tree structure
  for (const concept of concepts) {
    const node = nodeMap.get(concept.id)!;
    if (concept.parent_id && nodeMap.has(concept.parent_id)) {
      const parent = nodeMap.get(concept.parent_id)!;
      parent.children.push(node);
    } else {
      roots.push(node);
    }
  }

  // Sort children by sort_order
  const sortNodes = (nodes: TreeNode[]): TreeNode[] => {
    return nodes
      .sort((a, b) => {
        const orderA = a.concept.sort_order ?? 999;
        const orderB = b.concept.sort_order ?? 999;
        return orderA - orderB;
      })
      .map((node) => ({
        ...node,
        children: sortNodes(node.children),
      }));
  };

  return sortNodes(roots);
}

export function filterTree(nodes: TreeNode[], query: string): TreeNode[] {
  if (!query.trim()) return nodes;

  const lowerQuery = query.toLowerCase();

  const filterNode = (node: TreeNode): TreeNode | null => {
    const matchesName = node.name.toLowerCase().includes(lowerQuery);
    const matchesCode = node.code?.toLowerCase().includes(lowerQuery);
    const filteredChildren = node.children
      .map(filterNode)
      .filter((n): n is TreeNode => n !== null);

    if (matchesName || matchesCode || filteredChildren.length > 0) {
      return {
        ...node,
        children: filteredChildren,
        isExpanded: filteredChildren.length > 0,
      };
    }
    return null;
  };

  return nodes.map(filterNode).filter((n): n is TreeNode => n !== null);
}

export function findNodePath(nodes: TreeNode[], targetId: string): TreeNode[] {
  const path: TreeNode[] = [];

  const find = (nodeList: TreeNode[]): boolean => {
    for (const node of nodeList) {
      if (node.id === targetId) {
        path.push(node);
        return true;
      }
      if (find(node.children)) {
        path.unshift(node);
        return true;
      }
    }
    return false;
  };

  find(nodes);
  return path;
}
```

**Step 2: Create graph transform utility**

```typescript
// frontend/src/features/ontology/utils/graphTransform.ts
import type { Concept, Relationship, GraphNode, GraphEdge, GraphData } from "../types";

export function conceptsToGraphNodes(concepts: Concept[], language: "en" | "nb" = "en"): GraphNode[] {
  return concepts.map((concept) => ({
    id: concept.id,
    name: language === "nb" && concept.name_nb ? concept.name_nb : concept.name_en,
    type: concept.concept_type,
    frameworkId: concept.framework_id,
    code: concept.code,
  }));
}

export function relationshipsToGraphEdges(
  relationships: Relationship[],
  nodeIds: Set<string>
): GraphEdge[] {
  return relationships
    .filter(
      (rel) => nodeIds.has(rel.source_concept_id) && nodeIds.has(rel.target_concept_id)
    )
    .map((rel) => ({
      id: rel.id,
      source: rel.source_concept_id,
      target: rel.target_concept_id,
      type: rel.relationship_type,
      label: formatRelationshipLabel(rel.relationship_type),
    }));
}

export function formatRelationshipLabel(type: string): string {
  return type
    .replace(/_/g, " ")
    .replace(/([A-Z])/g, " $1")
    .trim()
    .toLowerCase();
}

export function buildGraphData(
  concepts: Concept[],
  relationships: Relationship[],
  language: "en" | "nb" = "en"
): GraphData {
  const nodes = conceptsToGraphNodes(concepts, language);
  const nodeIds = new Set(nodes.map((n) => n.id));
  const edges = relationshipsToGraphEdges(relationships, nodeIds);
  return { nodes, edges };
}

// Framework colors for consistent theming
export const frameworkColors: Record<string, string> = {
  iso31000: "#3b82f6", // blue
  iso31010: "#22c55e", // green
  "nist-csf": "#f97316", // orange
  iso27000: "#8b5cf6", // purple
  iso9000: "#ec4899", // pink
  "zero-trust": "#14b8a6", // teal
  "data-centric": "#eab308", // yellow
  fmn: "#6366f1", // indigo
};

export function getFrameworkColor(frameworkId: string): string {
  return frameworkColors[frameworkId] ?? "#64748b"; // slate fallback
}
```

**Step 3: Create barrel export**

```typescript
// frontend/src/features/ontology/utils/index.ts
export * from "./treeBuilder";
export * from "./graphTransform";
```

**Step 4: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 5: Commit**

```bash
git add frontend/src/features/ontology/utils/
git commit -m "feat(ontology): add tree builder and graph transform utilities"
```

---

## Task 5: Create Explorer Context

**Files:**
- Create: `frontend/src/features/ontology/context/ExplorerContext.tsx`
- Create: `frontend/src/features/ontology/context/index.ts`

**Step 1: Create context provider**

```typescript
// frontend/src/features/ontology/context/ExplorerContext.tsx
import { createContext, useContext, useReducer, type ReactNode } from "react";
import type { ViewMode, ExplorerState } from "../types";

type ExplorerAction =
  | { type: "SELECT_CONCEPT"; conceptId: string | null }
  | { type: "TOGGLE_CONCEPT"; conceptId: string }
  | { type: "CLEAR_SELECTION" }
  | { type: "SET_VIEW_MODE"; mode: ViewMode }
  | { type: "TOGGLE_SIDEBAR" }
  | { type: "SET_COMPARE_LEFT"; frameworkId: string | null }
  | { type: "SET_COMPARE_RIGHT"; frameworkId: string | null };

const initialState: ExplorerState = {
  selectedConceptId: null,
  selectedConcepts: [],
  viewMode: "graph",
  sidebarCollapsed: false,
  compareFrameworks: [null, null],
};

function explorerReducer(state: ExplorerState, action: ExplorerAction): ExplorerState {
  switch (action.type) {
    case "SELECT_CONCEPT":
      return {
        ...state,
        selectedConceptId: action.conceptId,
        selectedConcepts: action.conceptId ? [action.conceptId] : [],
      };
    case "TOGGLE_CONCEPT": {
      const isSelected = state.selectedConcepts.includes(action.conceptId);
      return {
        ...state,
        selectedConceptId: action.conceptId,
        selectedConcepts: isSelected
          ? state.selectedConcepts.filter((id) => id !== action.conceptId)
          : [...state.selectedConcepts, action.conceptId],
      };
    }
    case "CLEAR_SELECTION":
      return {
        ...state,
        selectedConceptId: null,
        selectedConcepts: [],
      };
    case "SET_VIEW_MODE":
      return { ...state, viewMode: action.mode };
    case "TOGGLE_SIDEBAR":
      return { ...state, sidebarCollapsed: !state.sidebarCollapsed };
    case "SET_COMPARE_LEFT":
      return {
        ...state,
        compareFrameworks: [action.frameworkId, state.compareFrameworks[1]],
      };
    case "SET_COMPARE_RIGHT":
      return {
        ...state,
        compareFrameworks: [state.compareFrameworks[0], action.frameworkId],
      };
    default:
      return state;
  }
}

interface ExplorerContextValue {
  state: ExplorerState;
  selectConcept: (conceptId: string | null) => void;
  toggleConceptSelection: (conceptId: string) => void;
  clearSelection: () => void;
  setViewMode: (mode: ViewMode) => void;
  toggleSidebar: () => void;
  setCompareLeft: (frameworkId: string | null) => void;
  setCompareRight: (frameworkId: string | null) => void;
}

const ExplorerContext = createContext<ExplorerContextValue | null>(null);

export function ExplorerProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(explorerReducer, initialState);

  const value: ExplorerContextValue = {
    state,
    selectConcept: (conceptId) => dispatch({ type: "SELECT_CONCEPT", conceptId }),
    toggleConceptSelection: (conceptId) => dispatch({ type: "TOGGLE_CONCEPT", conceptId }),
    clearSelection: () => dispatch({ type: "CLEAR_SELECTION" }),
    setViewMode: (mode) => dispatch({ type: "SET_VIEW_MODE", mode }),
    toggleSidebar: () => dispatch({ type: "TOGGLE_SIDEBAR" }),
    setCompareLeft: (frameworkId) => dispatch({ type: "SET_COMPARE_LEFT", frameworkId }),
    setCompareRight: (frameworkId) => dispatch({ type: "SET_COMPARE_RIGHT", frameworkId }),
  };

  return (
    <ExplorerContext.Provider value={value}>{children}</ExplorerContext.Provider>
  );
}

export function useExplorer() {
  const context = useContext(ExplorerContext);
  if (!context) {
    throw new Error("useExplorer must be used within an ExplorerProvider");
  }
  return context;
}
```

**Step 2: Create barrel export**

```typescript
// frontend/src/features/ontology/context/index.ts
export { ExplorerProvider, useExplorer } from "./ExplorerContext";
```

**Step 3: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 4: Commit**

```bash
git add frontend/src/features/ontology/context/
git commit -m "feat(ontology): add explorer context for UI state management"
```

---

## Task 6: Create Sidebar Components

**Files:**
- Create: `frontend/src/features/ontology/components/Sidebar/TreeNode.tsx`
- Create: `frontend/src/features/ontology/components/Sidebar/FrameworkTree.tsx`
- Create: `frontend/src/features/ontology/components/Sidebar/SearchBox.tsx`
- Create: `frontend/src/features/ontology/components/Sidebar/Sidebar.tsx`
- Create: `frontend/src/features/ontology/components/Sidebar/index.ts`

**Step 1: Create TreeNode component**

```typescript
// frontend/src/features/ontology/components/Sidebar/TreeNode.tsx
import { useState } from "react";
import { ChevronRight, Folder, Diamond, ArrowRight, Cog } from "lucide-react";
import { cn } from "@/lib/utils";
import type { TreeNode as TreeNodeType } from "../../types";
import { useExplorer } from "../../context";

interface TreeNodeProps {
  node: TreeNodeType;
  level: number;
}

const typeIcons: Record<string, typeof Folder> = {
  category: Folder,
  principle: Diamond,
  process: ArrowRight,
  technique: Cog,
  framework_component: Folder,
  function: Folder,
  subcategory: Folder,
};

export function TreeNode({ node, level }: TreeNodeProps) {
  const [isExpanded, setIsExpanded] = useState(node.isExpanded);
  const { state, selectConcept } = useExplorer();
  const isSelected = state.selectedConceptId === node.id;
  const hasChildren = node.children.length > 0;

  const Icon = typeIcons[node.type] ?? Folder;

  const handleClick = () => {
    selectConcept(node.id);
  };

  const handleDoubleClick = () => {
    if (hasChildren) {
      setIsExpanded(!isExpanded);
    }
  };

  const handleChevronClick = (e: React.MouseEvent) => {
    e.stopPropagation();
    setIsExpanded(!isExpanded);
  };

  return (
    <div>
      <div
        className={cn(
          "flex items-center gap-1 py-1 px-2 cursor-pointer rounded text-sm",
          "hover:bg-accent/50 transition-colors",
          isSelected && "bg-accent text-accent-foreground"
        )}
        style={{ paddingLeft: `${level * 12 + 8}px` }}
        onClick={handleClick}
        onDoubleClick={handleDoubleClick}
      >
        {hasChildren ? (
          <button
            onClick={handleChevronClick}
            className="p-0.5 hover:bg-accent rounded"
          >
            <ChevronRight
              className={cn(
                "h-3 w-3 transition-transform",
                isExpanded && "rotate-90"
              )}
            />
          </button>
        ) : (
          <span className="w-4" />
        )}
        <Icon className="h-3.5 w-3.5 text-muted-foreground shrink-0" />
        <span className="truncate">
          {node.code && (
            <span className="text-muted-foreground mr-1">{node.code}</span>
          )}
          {node.name}
        </span>
      </div>
      {isExpanded && hasChildren && (
        <div>
          {node.children.map((child) => (
            <TreeNode key={child.id} node={child} level={level + 1} />
          ))}
        </div>
      )}
    </div>
  );
}
```

**Step 2: Create FrameworkTree component**

```typescript
// frontend/src/features/ontology/components/Sidebar/FrameworkTree.tsx
import { useState, useMemo } from "react";
import { ChevronRight, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import { useConcepts } from "../../api";
import { buildTree, filterTree } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";
import { TreeNode } from "./TreeNode";
import type { Framework } from "../../types";

interface FrameworkTreeProps {
  framework: Framework;
  filterQuery: string;
}

export function FrameworkTree({ framework, filterQuery }: FrameworkTreeProps) {
  const [isExpanded, setIsExpanded] = useState(false);
  const { i18n } = useTranslation();
  const { data: concepts, isLoading } = useConcepts(
    isExpanded ? framework.id : undefined
  );

  const tree = useMemo(() => {
    if (!concepts) return [];
    const language = i18n.language.startsWith("nb") ? "nb" : "en";
    const fullTree = buildTree(concepts, language);
    return filterQuery ? filterTree(fullTree, filterQuery) : fullTree;
  }, [concepts, filterQuery, i18n.language]);

  const borderColor = getFrameworkColor(framework.id);

  return (
    <div className="border-l-2" style={{ borderColor }}>
      <button
        onClick={() => setIsExpanded(!isExpanded)}
        className={cn(
          "flex items-center gap-2 w-full py-2 px-3 text-left",
          "hover:bg-accent/50 transition-colors font-medium text-sm"
        )}
      >
        <ChevronRight
          className={cn(
            "h-4 w-4 transition-transform shrink-0",
            isExpanded && "rotate-90"
          )}
        />
        <span className="truncate">{framework.name}</span>
        {isLoading && <Loader2 className="h-3 w-3 animate-spin ml-auto" />}
      </button>
      {isExpanded && (
        <div className="pb-2">
          {tree.length === 0 && !isLoading && (
            <div className="px-4 py-2 text-sm text-muted-foreground">
              {filterQuery ? "No matches" : "No concepts"}
            </div>
          )}
          {tree.map((node) => (
            <TreeNode key={node.id} node={node} level={1} />
          ))}
        </div>
      )}
    </div>
  );
}
```

**Step 3: Create SearchBox component**

```typescript
// frontend/src/features/ontology/components/Sidebar/SearchBox.tsx
import { useState } from "react";
import { Search, Loader2 } from "lucide-react";
import { useTranslation } from "react-i18next";
import { useSearchConcepts } from "../../api";
import { useExplorer } from "../../context";
import { cn } from "@/lib/utils";

export function SearchBox() {
  const { t } = useTranslation("ontology");
  const [query, setQuery] = useState("");
  const [isOpen, setIsOpen] = useState(false);
  const { selectConcept, setViewMode } = useExplorer();
  const { data: results, isLoading } = useSearchConcepts(query);

  const handleSelect = (conceptId: string) => {
    selectConcept(conceptId);
    setViewMode("detail");
    setQuery("");
    setIsOpen(false);
  };

  return (
    <div className="relative">
      <div className="flex items-center gap-2 px-3 py-2 border-t">
        <Search className="h-4 w-4 text-muted-foreground" />
        <input
          type="text"
          value={query}
          onChange={(e) => {
            setQuery(e.target.value);
            setIsOpen(true);
          }}
          onFocus={() => setIsOpen(true)}
          placeholder={t("concepts.search")}
          className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
        {isLoading && <Loader2 className="h-4 w-4 animate-spin" />}
      </div>
      {isOpen && query.length >= 2 && results && results.length > 0 && (
        <div className="absolute bottom-full left-0 right-0 mb-1 bg-popover border rounded-md shadow-lg max-h-64 overflow-y-auto z-50">
          {results.map((concept) => (
            <button
              key={concept.id}
              onClick={() => handleSelect(concept.id)}
              className={cn(
                "w-full px-3 py-2 text-left text-sm",
                "hover:bg-accent transition-colors",
                "border-b last:border-b-0"
              )}
            >
              <div className="font-medium">{concept.name_en}</div>
              <div className="text-xs text-muted-foreground">
                {concept.framework_id} · {concept.concept_type}
              </div>
            </button>
          ))}
        </div>
      )}
    </div>
  );
}
```

**Step 4: Create Sidebar component**

```typescript
// frontend/src/features/ontology/components/Sidebar/Sidebar.tsx
import { useState } from "react";
import { PanelLeftClose, PanelLeft, Filter } from "lucide-react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import { useFrameworks } from "../../api";
import { useExplorer } from "../../context";
import { FrameworkTree } from "./FrameworkTree";
import { SearchBox } from "./SearchBox";

export function Sidebar() {
  const { t } = useTranslation("ontology");
  const [filterQuery, setFilterQuery] = useState("");
  const { state, toggleSidebar } = useExplorer();
  const { data: frameworks, isLoading } = useFrameworks();

  if (state.sidebarCollapsed) {
    return (
      <div className="w-10 border-r bg-card flex flex-col items-center py-2">
        <button
          onClick={toggleSidebar}
          className="p-2 hover:bg-accent rounded"
          title="Expand sidebar"
        >
          <PanelLeft className="h-4 w-4" />
        </button>
      </div>
    );
  }

  return (
    <div className="w-72 border-r bg-card flex flex-col">
      {/* Header */}
      <div className="flex items-center justify-between px-3 py-2 border-b">
        <h2 className="font-semibold text-sm">{t("frameworks.title")}</h2>
        <button
          onClick={toggleSidebar}
          className="p-1 hover:bg-accent rounded"
          title="Collapse sidebar"
        >
          <PanelLeftClose className="h-4 w-4" />
        </button>
      </div>

      {/* Filter input */}
      <div className="flex items-center gap-2 px-3 py-2 border-b">
        <Filter className="h-4 w-4 text-muted-foreground" />
        <input
          type="text"
          value={filterQuery}
          onChange={(e) => setFilterQuery(e.target.value)}
          placeholder="Filter..."
          className="flex-1 bg-transparent text-sm outline-none placeholder:text-muted-foreground"
        />
      </div>

      {/* Framework trees */}
      <div className="flex-1 overflow-y-auto">
        {isLoading ? (
          <div className="p-4 text-sm text-muted-foreground">Loading...</div>
        ) : (
          frameworks?.map((framework) => (
            <FrameworkTree
              key={framework.id}
              framework={framework}
              filterQuery={filterQuery}
            />
          ))
        )}
      </div>

      {/* Search box */}
      <SearchBox />
    </div>
  );
}
```

**Step 5: Create barrel export**

```typescript
// frontend/src/features/ontology/components/Sidebar/index.ts
export { Sidebar } from "./Sidebar";
export { FrameworkTree } from "./FrameworkTree";
export { TreeNode } from "./TreeNode";
export { SearchBox } from "./SearchBox";
```

**Step 6: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 7: Commit**

```bash
git add frontend/src/features/ontology/components/Sidebar/
git commit -m "feat(ontology): add sidebar with framework tree and search"
```

---

## Task 7: Create Graph Components

**Files:**
- Create: `frontend/src/features/ontology/hooks/useD3Graph.ts`
- Create: `frontend/src/features/ontology/components/Graph/GraphControls.tsx`
- Create: `frontend/src/features/ontology/components/Graph/Minimap.tsx`
- Create: `frontend/src/features/ontology/components/Graph/GraphView.tsx`
- Create: `frontend/src/features/ontology/components/Graph/index.ts`

**Step 1: Create useD3Graph hook**

```typescript
// frontend/src/features/ontology/hooks/useD3Graph.ts
import { useEffect, useRef, useCallback } from "react";
import * as d3 from "d3";
import type { GraphData, GraphNode, GraphEdge } from "../types";
import { getFrameworkColor } from "../utils/graphTransform";

interface UseD3GraphOptions {
  data: GraphData;
  onNodeClick?: (node: GraphNode) => void;
  onNodeDoubleClick?: (node: GraphNode) => void;
  selectedNodeId?: string | null;
  width: number;
  height: number;
}

export function useD3Graph({
  data,
  onNodeClick,
  onNodeDoubleClick,
  selectedNodeId,
  width,
  height,
}: UseD3GraphOptions) {
  const svgRef = useRef<SVGSVGElement>(null);
  const simulationRef = useRef<d3.Simulation<GraphNode, GraphEdge> | null>(null);
  const zoomRef = useRef<d3.ZoomBehavior<SVGSVGElement, unknown> | null>(null);

  const zoomIn = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(300)
        .call(zoomRef.current.scaleBy, 1.3);
    }
  }, []);

  const zoomOut = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(300)
        .call(zoomRef.current.scaleBy, 0.7);
    }
  }, []);

  const resetView = useCallback(() => {
    if (svgRef.current && zoomRef.current) {
      d3.select(svgRef.current)
        .transition()
        .duration(500)
        .call(zoomRef.current.transform, d3.zoomIdentity);
    }
  }, []);

  const fitToScreen = useCallback(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    const bounds = {
      minX: d3.min(data.nodes, (d) => d.x ?? 0) ?? 0,
      maxX: d3.max(data.nodes, (d) => d.x ?? 0) ?? width,
      minY: d3.min(data.nodes, (d) => d.y ?? 0) ?? 0,
      maxY: d3.max(data.nodes, (d) => d.y ?? 0) ?? height,
    };

    const graphWidth = bounds.maxX - bounds.minX + 100;
    const graphHeight = bounds.maxY - bounds.minY + 100;
    const scale = Math.min(width / graphWidth, height / graphHeight, 1) * 0.9;
    const translateX = (width - graphWidth * scale) / 2 - bounds.minX * scale + 50;
    const translateY = (height - graphHeight * scale) / 2 - bounds.minY * scale + 50;

    if (zoomRef.current) {
      svg
        .transition()
        .duration(500)
        .call(
          zoomRef.current.transform,
          d3.zoomIdentity.translate(translateX, translateY).scale(scale)
        );
    }
  }, [data.nodes, width, height]);

  useEffect(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    // Create container group for zoom
    const g = svg.append("g");

    // Setup zoom
    const zoom = d3.zoom<SVGSVGElement, unknown>()
      .scaleExtent([0.1, 4])
      .on("zoom", (event) => {
        g.attr("transform", event.transform);
      });

    svg.call(zoom);
    zoomRef.current = zoom;

    // Create arrow marker
    svg.append("defs").append("marker")
      .attr("id", "arrowhead")
      .attr("viewBox", "-0 -5 10 10")
      .attr("refX", 20)
      .attr("refY", 0)
      .attr("orient", "auto")
      .attr("markerWidth", 6)
      .attr("markerHeight", 6)
      .append("path")
      .attr("d", "M 0,-5 L 10,0 L 0,5")
      .attr("fill", "#94a3b8");

    // Create links
    const link = g.append("g")
      .selectAll("path")
      .data(data.edges)
      .join("path")
      .attr("stroke", "#94a3b8")
      .attr("stroke-width", 1.5)
      .attr("fill", "none")
      .attr("marker-end", "url(#arrowhead)")
      .attr("opacity", 0.6);

    // Create link labels
    const linkLabel = g.append("g")
      .selectAll("text")
      .data(data.edges)
      .join("text")
      .attr("font-size", 10)
      .attr("fill", "#64748b")
      .attr("text-anchor", "middle")
      .text((d) => d.label);

    // Create nodes
    const node = g.append("g")
      .selectAll("g")
      .data(data.nodes)
      .join("g")
      .attr("cursor", "pointer")
      .call(
        d3.drag<SVGGElement, GraphNode>()
          .on("start", (event, d) => {
            if (!event.active) simulationRef.current?.alphaTarget(0.3).restart();
            d.fx = d.x;
            d.fy = d.y;
          })
          .on("drag", (event, d) => {
            d.fx = event.x;
            d.fy = event.y;
          })
          .on("end", (event, d) => {
            if (!event.active) simulationRef.current?.alphaTarget(0);
            // Keep position fixed after drag
          })
      );

    // Node background
    node.append("rect")
      .attr("width", 140)
      .attr("height", 36)
      .attr("x", -70)
      .attr("y", -18)
      .attr("rx", 6)
      .attr("fill", "white")
      .attr("stroke", (d) => getFrameworkColor(d.frameworkId))
      .attr("stroke-width", 2);

    // Node text
    node.append("text")
      .attr("text-anchor", "middle")
      .attr("dominant-baseline", "middle")
      .attr("font-size", 11)
      .attr("fill", "#1e293b")
      .text((d) => {
        const text = d.code ? `${d.code} ${d.name}` : d.name;
        return text.length > 18 ? text.slice(0, 16) + "..." : text;
      });

    // Node interactions
    node
      .on("click", (event, d) => {
        event.stopPropagation();
        onNodeClick?.(d);
      })
      .on("dblclick", (event, d) => {
        event.stopPropagation();
        onNodeDoubleClick?.(d);
      })
      .on("mouseenter", function (event, d) {
        d3.select(this).select("rect").attr("filter", "drop-shadow(0 4px 6px rgb(0 0 0 / 0.1))");
        // Highlight connected edges
        link.attr("opacity", (l) =>
          (l.source as GraphNode).id === d.id || (l.target as GraphNode).id === d.id ? 1 : 0.2
        );
      })
      .on("mouseleave", function () {
        d3.select(this).select("rect").attr("filter", null);
        link.attr("opacity", 0.6);
      });

    // Highlight selected node
    node.select("rect")
      .attr("fill", (d) => d.id === selectedNodeId ? getFrameworkColor(d.frameworkId) : "white")
      .attr("stroke-width", (d) => d.id === selectedNodeId ? 3 : 2);

    node.select("text")
      .attr("fill", (d) => d.id === selectedNodeId ? "white" : "#1e293b");

    // Setup simulation
    const simulation = d3.forceSimulation(data.nodes)
      .force("link", d3.forceLink<GraphNode, GraphEdge>(data.edges)
        .id((d) => d.id)
        .distance(150))
      .force("charge", d3.forceManyBody().strength(-400))
      .force("center", d3.forceCenter(width / 2, height / 2))
      .force("collision", d3.forceCollide().radius(80));

    simulationRef.current = simulation;

    simulation.on("tick", () => {
      link.attr("d", (d) => {
        const source = d.source as GraphNode;
        const target = d.target as GraphNode;
        const dx = (target.x ?? 0) - (source.x ?? 0);
        const dy = (target.y ?? 0) - (source.y ?? 0);
        const dr = Math.sqrt(dx * dx + dy * dy) * 1.5;
        return `M${source.x},${source.y}A${dr},${dr} 0 0,1 ${target.x},${target.y}`;
      });

      linkLabel
        .attr("x", (d) => {
          const source = d.source as GraphNode;
          const target = d.target as GraphNode;
          return ((source.x ?? 0) + (target.x ?? 0)) / 2;
        })
        .attr("y", (d) => {
          const source = d.source as GraphNode;
          const target = d.target as GraphNode;
          return ((source.y ?? 0) + (target.y ?? 0)) / 2 - 5;
        });

      node.attr("transform", (d) => `translate(${d.x},${d.y})`);
    });

    // Click on background to deselect
    svg.on("click", () => onNodeClick?.(null as unknown as GraphNode));

    return () => {
      simulation.stop();
    };
  }, [data, width, height, selectedNodeId, onNodeClick, onNodeDoubleClick]);

  return {
    svgRef,
    zoomIn,
    zoomOut,
    resetView,
    fitToScreen,
  };
}
```

**Step 2: Create GraphControls component**

```typescript
// frontend/src/features/ontology/components/Graph/GraphControls.tsx
import { ZoomIn, ZoomOut, RotateCcw, Maximize, Map } from "lucide-react";
import { cn } from "@/lib/utils";

interface GraphControlsProps {
  onZoomIn: () => void;
  onZoomOut: () => void;
  onResetView: () => void;
  onFitToScreen: () => void;
  onToggleMinimap: () => void;
  minimapVisible: boolean;
}

export function GraphControls({
  onZoomIn,
  onZoomOut,
  onResetView,
  onFitToScreen,
  onToggleMinimap,
  minimapVisible,
}: GraphControlsProps) {
  const buttonClass = cn(
    "p-2 bg-card border rounded-md shadow-sm",
    "hover:bg-accent transition-colors",
    "focus:outline-none focus:ring-2 focus:ring-ring"
  );

  return (
    <div className="absolute bottom-4 right-4 flex flex-col gap-1">
      <div className="flex gap-1">
        <button onClick={onZoomIn} className={buttonClass} title="Zoom in (+)">
          <ZoomIn className="h-4 w-4" />
        </button>
        <button onClick={onZoomOut} className={buttonClass} title="Zoom out (-)">
          <ZoomOut className="h-4 w-4" />
        </button>
        <button onClick={onResetView} className={buttonClass} title="Reset view (0)">
          <RotateCcw className="h-4 w-4" />
        </button>
      </div>
      <div className="flex gap-1">
        <button onClick={onFitToScreen} className={buttonClass} title="Fit to screen">
          <Maximize className="h-4 w-4" />
        </button>
        <button
          onClick={onToggleMinimap}
          className={cn(buttonClass, minimapVisible && "bg-accent")}
          title="Toggle minimap"
        >
          <Map className="h-4 w-4" />
        </button>
      </div>
    </div>
  );
}
```

**Step 3: Create Minimap component**

```typescript
// frontend/src/features/ontology/components/Graph/Minimap.tsx
import { useEffect, useRef } from "react";
import * as d3 from "d3";
import type { GraphData, GraphNode } from "../../types";
import { getFrameworkColor } from "../../utils/graphTransform";

interface MinimapProps {
  data: GraphData;
  width: number;
  height: number;
  viewportBounds?: { x: number; y: number; width: number; height: number };
}

export function Minimap({ data, width, height }: MinimapProps) {
  const svgRef = useRef<SVGSVGElement>(null);
  const minimapWidth = 150;
  const minimapHeight = 100;

  useEffect(() => {
    if (!svgRef.current || !data.nodes.length) return;

    const svg = d3.select(svgRef.current);
    svg.selectAll("*").remove();

    // Calculate scale to fit all nodes
    const bounds = {
      minX: d3.min(data.nodes, (d) => d.x ?? 0) ?? 0,
      maxX: d3.max(data.nodes, (d) => d.x ?? 0) ?? width,
      minY: d3.min(data.nodes, (d) => d.y ?? 0) ?? 0,
      maxY: d3.max(data.nodes, (d) => d.y ?? 0) ?? height,
    };

    const graphWidth = bounds.maxX - bounds.minX + 50;
    const graphHeight = bounds.maxY - bounds.minY + 50;
    const scale = Math.min(minimapWidth / graphWidth, minimapHeight / graphHeight) * 0.9;

    const g = svg.append("g")
      .attr("transform", `translate(${minimapWidth / 2 - (bounds.minX + graphWidth / 2) * scale}, ${minimapHeight / 2 - (bounds.minY + graphHeight / 2) * scale}) scale(${scale})`);

    // Draw edges
    g.selectAll("line")
      .data(data.edges)
      .join("line")
      .attr("x1", (d) => (d.source as GraphNode).x ?? 0)
      .attr("y1", (d) => (d.source as GraphNode).y ?? 0)
      .attr("x2", (d) => (d.target as GraphNode).x ?? 0)
      .attr("y2", (d) => (d.target as GraphNode).y ?? 0)
      .attr("stroke", "#94a3b8")
      .attr("stroke-width", 1 / scale);

    // Draw nodes
    g.selectAll("circle")
      .data(data.nodes)
      .join("circle")
      .attr("cx", (d) => d.x ?? 0)
      .attr("cy", (d) => d.y ?? 0)
      .attr("r", 4 / scale)
      .attr("fill", (d) => getFrameworkColor(d.frameworkId));

  }, [data, width, height]);

  return (
    <div className="absolute top-4 right-4 bg-card/90 border rounded-md shadow-sm p-1">
      <svg
        ref={svgRef}
        width={minimapWidth}
        height={minimapHeight}
        className="bg-muted/30 rounded"
      />
    </div>
  );
}
```

**Step 4: Create GraphView component**

```typescript
// frontend/src/features/ontology/components/Graph/GraphView.tsx
import { useState, useMemo, useEffect, useRef } from "react";
import { useTranslation } from "react-i18next";
import { useConcepts, useRelationships } from "../../api";
import { useExplorer } from "../../context";
import { buildGraphData } from "../../utils/graphTransform";
import { useD3Graph } from "../../hooks/useD3Graph";
import { GraphControls } from "./GraphControls";
import { Minimap } from "./Minimap";
import type { GraphNode } from "../../types";

export function GraphView() {
  const containerRef = useRef<HTMLDivElement>(null);
  const [dimensions, setDimensions] = useState({ width: 800, height: 600 });
  const [showMinimap, setShowMinimap] = useState(true);
  const { i18n } = useTranslation();
  const { state, selectConcept, setViewMode } = useExplorer();

  // Fetch all concepts from all frameworks for now
  // In production, you might want to paginate or filter
  const { data: iso31000Concepts } = useConcepts("iso31000");
  const { data: iso31010Concepts } = useConcepts("iso31010");
  const { data: nistCsfConcepts } = useConcepts("nist-csf");
  const { data: relationships } = useRelationships();

  // Combine concepts from different frameworks
  const allConcepts = useMemo(() => {
    const concepts = [
      ...(iso31000Concepts ?? []),
      ...(iso31010Concepts ?? []),
      ...(nistCsfConcepts ?? []),
    ];
    // Limit to top-level concepts for initial view
    return concepts.filter((c) => !c.parent_id).slice(0, 50);
  }, [iso31000Concepts, iso31010Concepts, nistCsfConcepts]);

  const graphData = useMemo(() => {
    const language = i18n.language.startsWith("nb") ? "nb" : "en";
    return buildGraphData(allConcepts, relationships ?? [], language);
  }, [allConcepts, relationships, i18n.language]);

  const handleNodeClick = (node: GraphNode | null) => {
    if (node) {
      selectConcept(node.id);
    } else {
      selectConcept(null);
    }
  };

  const handleNodeDoubleClick = (node: GraphNode) => {
    selectConcept(node.id);
    setViewMode("detail");
  };

  const { svgRef, zoomIn, zoomOut, resetView, fitToScreen } = useD3Graph({
    data: graphData,
    onNodeClick: handleNodeClick,
    onNodeDoubleClick: handleNodeDoubleClick,
    selectedNodeId: state.selectedConceptId,
    width: dimensions.width,
    height: dimensions.height,
  });

  // Handle resize
  useEffect(() => {
    const handleResize = () => {
      if (containerRef.current) {
        setDimensions({
          width: containerRef.current.clientWidth,
          height: containerRef.current.clientHeight,
        });
      }
    };

    handleResize();
    window.addEventListener("resize", handleResize);
    return () => window.removeEventListener("resize", handleResize);
  }, []);

  // Keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.target instanceof HTMLInputElement) return;

      switch (e.key) {
        case "+":
        case "=":
          zoomIn();
          break;
        case "-":
          zoomOut();
          break;
        case "0":
          resetView();
          break;
        case "Escape":
          selectConcept(null);
          break;
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [zoomIn, zoomOut, resetView, selectConcept]);

  return (
    <div ref={containerRef} className="relative w-full h-full bg-muted/20">
      <svg
        ref={svgRef}
        width={dimensions.width}
        height={dimensions.height}
        className="w-full h-full"
      />
      <GraphControls
        onZoomIn={zoomIn}
        onZoomOut={zoomOut}
        onResetView={resetView}
        onFitToScreen={fitToScreen}
        onToggleMinimap={() => setShowMinimap(!showMinimap)}
        minimapVisible={showMinimap}
      />
      {showMinimap && (
        <Minimap
          data={graphData}
          width={dimensions.width}
          height={dimensions.height}
        />
      )}
    </div>
  );
}
```

**Step 5: Create barrel export**

```typescript
// frontend/src/features/ontology/components/Graph/index.ts
export { GraphView } from "./GraphView";
export { GraphControls } from "./GraphControls";
export { Minimap } from "./Minimap";
```

**Step 6: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 7: Commit**

```bash
git add frontend/src/features/ontology/hooks/useD3Graph.ts
git add frontend/src/features/ontology/components/Graph/
git commit -m "feat(ontology): add D3.js graph visualization with controls and minimap"
```

---

## Task 8: Create Detail View

**Files:**
- Create: `frontend/src/features/ontology/components/Detail/DetailView.tsx`
- Create: `frontend/src/features/ontology/components/Detail/index.ts`

**Step 1: Create DetailView component**

```typescript
// frontend/src/features/ontology/components/Detail/DetailView.tsx
import { ArrowLeft, ExternalLink, ChevronRight } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { useConceptRelationships, useFramework } from "../../api";
import { useExplorer } from "../../context";
import { findNodePath, buildTree } from "../../utils";
import { useConcepts } from "../../api";
import { getFrameworkColor } from "../../utils/graphTransform";

export function DetailView() {
  const { t, i18n } = useTranslation("ontology");
  const { state, selectConcept, setViewMode } = useExplorer();
  const selectedId = state.selectedConceptId;

  const { data: conceptData, isLoading } = useConceptRelationships(selectedId ?? "");
  const { data: framework } = useFramework(conceptData?.framework_id ?? "");
  const { data: frameworkConcepts } = useConcepts(conceptData?.framework_id);

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  // Build breadcrumb path
  const breadcrumbPath = (() => {
    if (!frameworkConcepts || !selectedId) return [];
    const tree = buildTree(frameworkConcepts, language);
    return findNodePath(tree, selectedId);
  })();

  if (!selectedId) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Select a concept to view details
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Loading...
      </div>
    );
  }

  if (!conceptData) {
    return (
      <div className="flex items-center justify-center h-full text-muted-foreground">
        Concept not found
      </div>
    );
  }

  const name = language === "nb" && conceptData.name_nb ? conceptData.name_nb : conceptData.name_en;
  const definition = language === "nb" && conceptData.definition_nb
    ? conceptData.definition_nb
    : conceptData.definition_en;

  const borderColor = getFrameworkColor(conceptData.framework_id);

  const handleRelationshipClick = (conceptId: string) => {
    selectConcept(conceptId);
  };

  const handleViewInGraph = () => {
    setViewMode("graph");
  };

  return (
    <div className="h-full overflow-y-auto p-6">
      <div className="max-w-2xl mx-auto space-y-6">
        {/* Header */}
        <div className="flex items-center justify-between">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setViewMode("graph")}
          >
            <ArrowLeft className="h-4 w-4 mr-2" />
            Back to Graph
          </Button>
          <div className="flex gap-2">
            <Button
              variant={language === "en" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => i18n.changeLanguage("en")}
            >
              EN
            </Button>
            <Button
              variant={language === "nb" ? "secondary" : "ghost"}
              size="sm"
              onClick={() => i18n.changeLanguage("nb")}
            >
              NB
            </Button>
          </div>
        </div>

        {/* Main card */}
        <Card className="border-l-4" style={{ borderLeftColor: borderColor }}>
          <CardHeader>
            <div className="flex items-start justify-between">
              <div>
                <CardTitle className="text-xl">
                  {conceptData.code && (
                    <span className="text-muted-foreground mr-2">{conceptData.code}</span>
                  )}
                  {name}
                </CardTitle>
                <p className="text-sm text-muted-foreground mt-1">
                  {conceptData.concept_type}
                </p>
              </div>
              <span
                className="px-2 py-1 text-xs rounded-full text-white"
                style={{ backgroundColor: borderColor }}
              >
                {framework?.name ?? conceptData.framework_id}
              </span>
            </div>
          </CardHeader>
          <CardContent className="space-y-6">
            {/* Definition */}
            {definition && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Definition
                </h3>
                <p className="text-sm leading-relaxed">{definition}</p>
              </div>
            )}

            {/* Source */}
            {conceptData.source_reference && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Source
                </h3>
                <p className="text-sm">{conceptData.source_reference}</p>
              </div>
            )}

            {/* Relationships */}
            {conceptData.related_concepts.length > 0 && (
              <div>
                <div className="flex items-center justify-between mb-2">
                  <h3 className="font-medium text-sm text-muted-foreground">
                    {t("concepts.relationships")}
                  </h3>
                  <Button variant="ghost" size="sm" onClick={handleViewInGraph}>
                    View <ExternalLink className="h-3 w-3 ml-1" />
                  </Button>
                </div>
                <div className="space-y-1">
                  {conceptData.related_concepts.map((rel) => (
                    <button
                      key={rel.relationship_id}
                      onClick={() => handleRelationshipClick(rel.concept_id)}
                      className={cn(
                        "w-full flex items-center gap-2 p-2 text-sm rounded",
                        "hover:bg-accent transition-colors text-left"
                      )}
                    >
                      <span className="text-muted-foreground">
                        {rel.direction === "outgoing" ? "→" : "←"}
                      </span>
                      <span className="text-muted-foreground text-xs">
                        {rel.relationship_type}
                      </span>
                      <span className="flex-1">
                        {language === "nb" && rel.concept_name_nb
                          ? rel.concept_name_nb
                          : rel.concept_name_en}
                      </span>
                    </button>
                  ))}
                </div>
              </div>
            )}

            {/* Breadcrumb */}
            {breadcrumbPath.length > 0 && (
              <div>
                <h3 className="font-medium text-sm text-muted-foreground mb-2">
                  Hierarchy
                </h3>
                <div className="flex items-center flex-wrap gap-1 text-sm">
                  <span className="text-muted-foreground">
                    {framework?.name ?? conceptData.framework_id}
                  </span>
                  {breadcrumbPath.map((node, index) => (
                    <span key={node.id} className="flex items-center">
                      <ChevronRight className="h-3 w-3 mx-1 text-muted-foreground" />
                      {index === breadcrumbPath.length - 1 ? (
                        <span className="font-medium">{node.name}</span>
                      ) : (
                        <button
                          onClick={() => selectConcept(node.id)}
                          className="hover:underline"
                        >
                          {node.name}
                        </button>
                      )}
                    </span>
                  ))}
                </div>
              </div>
            )}
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
```

**Step 2: Create barrel export**

```typescript
// frontend/src/features/ontology/components/Detail/index.ts
export { DetailView } from "./DetailView";
```

**Step 3: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 4: Commit**

```bash
git add frontend/src/features/ontology/components/Detail/
git commit -m "feat(ontology): add concept detail view with relationships and breadcrumb"
```

---

## Task 9: Create Compare View

**Files:**
- Create: `frontend/src/features/ontology/components/Compare/CompareView.tsx`
- Create: `frontend/src/features/ontology/components/Compare/index.ts`

**Step 1: Create CompareView component**

```typescript
// frontend/src/features/ontology/components/Compare/CompareView.tsx
import { useMemo } from "react";
import { useTranslation } from "react-i18next";
import { cn } from "@/lib/utils";
import { useFrameworks, useConcepts, useRelationships } from "../../api";
import { useExplorer } from "../../context";
import { buildTree } from "../../utils";
import { getFrameworkColor } from "../../utils/graphTransform";
import { TreeNode } from "../Sidebar/TreeNode";
import type { Relationship } from "../../types";

export function CompareView() {
  const { i18n } = useTranslation();
  const { state, setCompareLeft, setCompareRight, selectConcept } = useExplorer();
  const [leftFrameworkId, rightFrameworkId] = state.compareFrameworks;

  const { data: frameworks } = useFrameworks();
  const { data: leftConcepts } = useConcepts(leftFrameworkId ?? undefined);
  const { data: rightConcepts } = useConcepts(rightFrameworkId ?? undefined);
  const { data: relationships } = useRelationships();

  const language = i18n.language.startsWith("nb") ? "nb" : "en";

  const leftTree = useMemo(() => {
    if (!leftConcepts) return [];
    return buildTree(leftConcepts, language);
  }, [leftConcepts, language]);

  const rightTree = useMemo(() => {
    if (!rightConcepts) return [];
    return buildTree(rightConcepts, language);
  }, [rightConcepts, language]);

  // Find cross-framework relationships
  const crossMappings = useMemo(() => {
    if (!relationships || !leftFrameworkId || !rightFrameworkId) return [];

    const leftIds = new Set(leftConcepts?.map((c) => c.id) ?? []);
    const rightIds = new Set(rightConcepts?.map((c) => c.id) ?? []);

    return relationships.filter(
      (rel) =>
        (leftIds.has(rel.source_concept_id) && rightIds.has(rel.target_concept_id)) ||
        (rightIds.has(rel.source_concept_id) && leftIds.has(rel.target_concept_id))
    );
  }, [relationships, leftFrameworkId, rightFrameworkId, leftConcepts, rightConcepts]);

  const getMappingsForConcept = (conceptId: string): Relationship[] => {
    return crossMappings.filter(
      (rel) => rel.source_concept_id === conceptId || rel.target_concept_id === conceptId
    );
  };

  return (
    <div className="h-full flex flex-col">
      {/* Framework selectors */}
      <div className="flex items-center justify-between p-4 border-b">
        <select
          value={leftFrameworkId ?? ""}
          onChange={(e) => setCompareLeft(e.target.value || null)}
          className="px-3 py-2 border rounded-md bg-background"
        >
          <option value="">Select framework...</option>
          {frameworks?.map((fw) => (
            <option key={fw.id} value={fw.id}>
              {fw.name}
            </option>
          ))}
        </select>
        <div className="text-muted-foreground">
          ←→
        </div>
        <select
          value={rightFrameworkId ?? ""}
          onChange={(e) => setCompareRight(e.target.value || null)}
          className="px-3 py-2 border rounded-md bg-background"
        >
          <option value="">Select framework...</option>
          {frameworks?.map((fw) => (
            <option key={fw.id} value={fw.id}>
              {fw.name}
            </option>
          ))}
        </select>
      </div>

      {/* Comparison area */}
      <div className="flex-1 flex min-h-0">
        {/* Left tree */}
        <div
          className="flex-1 overflow-y-auto border-r border-l-4"
          style={{ borderLeftColor: leftFrameworkId ? getFrameworkColor(leftFrameworkId) : undefined }}
        >
          {leftFrameworkId ? (
            leftTree.length > 0 ? (
              <div className="py-2">
                {leftTree.map((node) => (
                  <div key={node.id}>
                    <TreeNode node={node} level={0} />
                    {getMappingsForConcept(node.id).length > 0 && (
                      <div className="ml-6 py-1">
                        {getMappingsForConcept(node.id).map((rel) => (
                          <div
                            key={rel.id}
                            className="flex items-center gap-2 text-xs text-muted-foreground"
                          >
                            <span className="w-2 h-px bg-amber-500" />
                            <span>{rel.relationship_type}</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-4 text-muted-foreground text-sm">
                Loading concepts...
              </div>
            )
          ) : (
            <div className="p-4 text-muted-foreground text-sm">
              Select a framework
            </div>
          )}
        </div>

        {/* Center mappings column */}
        <div className="w-24 bg-muted/30 flex flex-col items-center justify-center">
          <div className="text-xs text-muted-foreground text-center p-2">
            {crossMappings.length} mappings
          </div>
          {crossMappings.slice(0, 10).map((rel) => (
            <div
              key={rel.id}
              className="w-full h-px bg-amber-500/50 my-1"
              title={rel.relationship_type}
            />
          ))}
          {crossMappings.length > 10 && (
            <div className="text-xs text-muted-foreground">
              +{crossMappings.length - 10} more
            </div>
          )}
        </div>

        {/* Right tree */}
        <div
          className="flex-1 overflow-y-auto border-l border-r-4"
          style={{ borderRightColor: rightFrameworkId ? getFrameworkColor(rightFrameworkId) : undefined }}
        >
          {rightFrameworkId ? (
            rightTree.length > 0 ? (
              <div className="py-2">
                {rightTree.map((node) => (
                  <div key={node.id}>
                    <TreeNode node={node} level={0} />
                    {getMappingsForConcept(node.id).length > 0 && (
                      <div className="ml-6 py-1">
                        {getMappingsForConcept(node.id).map((rel) => (
                          <div
                            key={rel.id}
                            className="flex items-center gap-2 text-xs text-muted-foreground"
                          >
                            <span className="w-2 h-px bg-amber-500" />
                            <span>{rel.relationship_type}</span>
                          </div>
                        ))}
                      </div>
                    )}
                  </div>
                ))}
              </div>
            ) : (
              <div className="p-4 text-muted-foreground text-sm">
                Loading concepts...
              </div>
            )
          ) : (
            <div className="p-4 text-muted-foreground text-sm">
              Select a framework
            </div>
          )}
        </div>
      </div>
    </div>
  );
}
```

**Step 2: Create barrel export**

```typescript
// frontend/src/features/ontology/components/Compare/index.ts
export { CompareView } from "./CompareView";
```

**Step 3: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 4: Commit**

```bash
git add frontend/src/features/ontology/components/Compare/
git commit -m "feat(ontology): add framework comparison view with cross-mappings"
```

---

## Task 10: Create Export Dialog

**Files:**
- Create: `frontend/src/features/ontology/components/ExportDialog.tsx`

**Step 1: Create ExportDialog component**

```typescript
// frontend/src/features/ontology/components/ExportDialog.tsx
import { useState } from "react";
import { X, Download } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { cn } from "@/lib/utils";

interface ExportDialogProps {
  isOpen: boolean;
  onClose: () => void;
  svgElement: SVGSVGElement | null;
}

export function ExportDialog({ isOpen, onClose, svgElement }: ExportDialogProps) {
  const [format, setFormat] = useState<"png" | "svg">("png");
  const [size, setSize] = useState<"current" | "full">("current");
  const [includeLegend, setIncludeLegend] = useState(true);
  const [includeTitle, setIncludeTitle] = useState(true);
  const [isExporting, setIsExporting] = useState(false);

  if (!isOpen) return null;

  const handleExport = async () => {
    if (!svgElement) return;
    setIsExporting(true);

    try {
      if (format === "svg") {
        // Export as SVG
        const serializer = new XMLSerializer();
        const svgString = serializer.serializeToString(svgElement);
        const blob = new Blob([svgString], { type: "image/svg+xml" });
        const url = URL.createObjectURL(blob);

        const link = document.createElement("a");
        link.href = url;
        link.download = "ontology-graph.svg";
        link.click();
        URL.revokeObjectURL(url);
      } else {
        // Export as PNG using html2canvas
        const html2canvas = (await import("html2canvas")).default;
        const canvas = await html2canvas(svgElement as unknown as HTMLElement, {
          backgroundColor: "#ffffff",
          scale: 2,
        });

        const link = document.createElement("a");
        link.href = canvas.toDataURL("image/png");
        link.download = "ontology-graph.png";
        link.click();
      }

      onClose();
    } catch (error) {
      console.error("Export failed:", error);
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center">
      <div className="absolute inset-0 bg-black/50" onClick={onClose} />
      <Card className="relative z-10 w-80">
        <CardHeader className="flex flex-row items-center justify-between">
          <CardTitle className="text-lg">Export Graph</CardTitle>
          <button onClick={onClose} className="p-1 hover:bg-accent rounded">
            <X className="h-4 w-4" />
          </button>
        </CardHeader>
        <CardContent className="space-y-4">
          {/* Format */}
          <div>
            <label className="text-sm font-medium">Format</label>
            <div className="flex gap-4 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="format"
                  checked={format === "png"}
                  onChange={() => setFormat("png")}
                  className="w-4 h-4"
                />
                <span className="text-sm">PNG</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="format"
                  checked={format === "svg"}
                  onChange={() => setFormat("svg")}
                  className="w-4 h-4"
                />
                <span className="text-sm">SVG</span>
              </label>
            </div>
          </div>

          {/* Size */}
          <div>
            <label className="text-sm font-medium">Size</label>
            <div className="flex gap-4 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="size"
                  checked={size === "current"}
                  onChange={() => setSize("current")}
                  className="w-4 h-4"
                />
                <span className="text-sm">Current view</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="radio"
                  name="size"
                  checked={size === "full"}
                  onChange={() => setSize("full")}
                  className="w-4 h-4"
                />
                <span className="text-sm">Full graph</span>
              </label>
            </div>
          </div>

          {/* Include options */}
          <div>
            <label className="text-sm font-medium">Include</label>
            <div className="space-y-2 mt-2">
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={includeLegend}
                  onChange={(e) => setIncludeLegend(e.target.checked)}
                  className="w-4 h-4 rounded"
                />
                <span className="text-sm">Legend</span>
              </label>
              <label className="flex items-center gap-2 cursor-pointer">
                <input
                  type="checkbox"
                  checked={includeTitle}
                  onChange={(e) => setIncludeTitle(e.target.checked)}
                  className="w-4 h-4 rounded"
                />
                <span className="text-sm">Title</span>
              </label>
            </div>
          </div>

          {/* Actions */}
          <div className="flex justify-end gap-2 pt-2">
            <Button variant="outline" onClick={onClose}>
              Cancel
            </Button>
            <Button onClick={handleExport} disabled={isExporting}>
              <Download className="h-4 w-4 mr-2" />
              {isExporting ? "Exporting..." : "Export"}
            </Button>
          </div>
        </CardContent>
      </Card>
    </div>
  );
}
```

**Step 2: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 3: Commit**

```bash
git add frontend/src/features/ontology/components/ExportDialog.tsx
git commit -m "feat(ontology): add export dialog for PNG/SVG export"
```

---

## Task 11: Create Main Explorer Component

**Files:**
- Create: `frontend/src/features/ontology/components/OntologyExplorer.tsx`
- Create: `frontend/src/features/ontology/components/index.ts`

**Step 1: Create OntologyExplorer component**

```typescript
// frontend/src/features/ontology/components/OntologyExplorer.tsx
import { useState, useRef } from "react";
import { Save, Download, Keyboard } from "lucide-react";
import { useTranslation } from "react-i18next";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { ExplorerProvider, useExplorer } from "../context";
import { Sidebar } from "./Sidebar";
import { GraphView } from "./Graph";
import { DetailView } from "./Detail";
import { CompareView } from "./Compare";
import { ExportDialog } from "./ExportDialog";
import type { ViewMode } from "../types";

function ExplorerContent() {
  const { t } = useTranslation("ontology");
  const { state, setViewMode, selectConcept } = useExplorer();
  const [showExportDialog, setShowExportDialog] = useState(false);
  const [showKeyboardHelp, setShowKeyboardHelp] = useState(false);
  const graphSvgRef = useRef<SVGSVGElement | null>(null);

  const viewModes: { mode: ViewMode; label: string }[] = [
    { mode: "graph", label: "Graph" },
    { mode: "detail", label: "Detail" },
    { mode: "compare", label: "Compare" },
  ];

  const handleSaveLayout = () => {
    // Layout is auto-saved, but this could trigger a manual save
    console.log("Layout saved");
  };

  return (
    <div className="flex h-[calc(100vh-4rem)]">
      <Sidebar />

      <div className="flex-1 flex flex-col min-w-0">
        {/* Toolbar */}
        <div className="flex items-center justify-between px-4 py-2 border-b bg-card">
          {/* View toggle */}
          <div className="flex gap-1">
            {viewModes.map(({ mode, label }) => (
              <Button
                key={mode}
                variant={state.viewMode === mode ? "secondary" : "ghost"}
                size="sm"
                onClick={() => setViewMode(mode)}
              >
                {label}
              </Button>
            ))}
          </div>

          {/* Actions */}
          <div className="flex items-center gap-1">
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowKeyboardHelp(!showKeyboardHelp)}
              title="Keyboard shortcuts"
            >
              <Keyboard className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={handleSaveLayout}
              title="Save layout (Ctrl+S)"
            >
              <Save className="h-4 w-4" />
            </Button>
            <Button
              variant="ghost"
              size="sm"
              onClick={() => setShowExportDialog(true)}
              title="Export (Ctrl+E)"
            >
              <Download className="h-4 w-4" />
            </Button>
          </div>
        </div>

        {/* Keyboard help overlay */}
        {showKeyboardHelp && (
          <div className="absolute top-20 right-4 z-40 bg-card border rounded-lg shadow-lg p-4 w-64">
            <h3 className="font-semibold mb-2">Keyboard Shortcuts</h3>
            <div className="space-y-1 text-sm">
              <div className="flex justify-between">
                <span>Zoom in</span>
                <kbd className="px-1 bg-muted rounded">+</kbd>
              </div>
              <div className="flex justify-between">
                <span>Zoom out</span>
                <kbd className="px-1 bg-muted rounded">-</kbd>
              </div>
              <div className="flex justify-between">
                <span>Reset view</span>
                <kbd className="px-1 bg-muted rounded">0</kbd>
              </div>
              <div className="flex justify-between">
                <span>Clear selection</span>
                <kbd className="px-1 bg-muted rounded">Esc</kbd>
              </div>
              <div className="flex justify-between">
                <span>Search</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+F</kbd>
              </div>
              <div className="flex justify-between">
                <span>Save</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+S</kbd>
              </div>
              <div className="flex justify-between">
                <span>Export</span>
                <kbd className="px-1 bg-muted rounded">Ctrl+E</kbd>
              </div>
            </div>
          </div>
        )}

        {/* Main content area */}
        <div className="flex-1 min-h-0 relative">
          {state.viewMode === "graph" && <GraphView />}
          {state.viewMode === "detail" && <DetailView />}
          {state.viewMode === "compare" && <CompareView />}
        </div>

        {/* Status bar */}
        <div className="flex items-center justify-between px-4 py-1 border-t bg-muted/30 text-xs text-muted-foreground">
          <div>
            {state.selectedConceptId ? (
              <span>Selected: {state.selectedConceptId}</span>
            ) : (
              <span>No selection</span>
            )}
          </div>
          <div className="flex gap-4">
            <span>{state.selectedConcepts.length} selected</span>
          </div>
        </div>
      </div>

      <ExportDialog
        isOpen={showExportDialog}
        onClose={() => setShowExportDialog(false)}
        svgElement={graphSvgRef.current}
      />
    </div>
  );
}

export function OntologyExplorer() {
  return (
    <ExplorerProvider>
      <ExplorerContent />
    </ExplorerProvider>
  );
}
```

**Step 2: Create barrel exports**

```typescript
// frontend/src/features/ontology/components/index.ts
export { OntologyExplorer } from "./OntologyExplorer";
export { Sidebar } from "./Sidebar";
export { GraphView } from "./Graph";
export { DetailView } from "./Detail";
export { CompareView } from "./Compare";
export { ExportDialog } from "./ExportDialog";
```

**Step 3: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 4: Commit**

```bash
git add frontend/src/features/ontology/components/OntologyExplorer.tsx
git add frontend/src/features/ontology/components/index.ts
git commit -m "feat(ontology): add main OntologyExplorer component with toolbar and status bar"
```

---

## Task 12: Update Route and Integrate

**Files:**
- Modify: `frontend/src/routes/ontology/index.tsx`
- Modify: `frontend/src/i18n/locales/en/ontology.json`
- Modify: `frontend/src/i18n/locales/nb/ontology.json`

**Step 1: Update ontology route**

```typescript
// frontend/src/routes/ontology/index.tsx
import { createFileRoute } from "@tanstack/react-router";
import { OntologyExplorer } from "@/features/ontology/components";

export const Route = createFileRoute("/ontology/")({
  component: OntologyPage,
});

function OntologyPage() {
  return <OntologyExplorer />;
}
```

**Step 2: Update English translations**

```json
{
  "title": "Ontology Explorer",
  "frameworks": {
    "title": "Frameworks",
    "iso31000": "ISO 31000:2018 - Risk Management",
    "iso31010": "ISO 31010 - Risk Assessment Techniques",
    "nistCsf": "NIST Cybersecurity Framework"
  },
  "concepts": {
    "title": "Concepts",
    "search": "Search all concepts...",
    "filter": "Filter...",
    "properties": "Properties",
    "relationships": "Relationships",
    "noResults": "No concepts found"
  },
  "graph": {
    "title": "Concept Graph",
    "zoomIn": "Zoom In",
    "zoomOut": "Zoom Out",
    "resetView": "Reset View",
    "fitToScreen": "Fit to Screen",
    "toggleMinimap": "Toggle Minimap"
  },
  "detail": {
    "backToGraph": "Back to Graph",
    "definition": "Definition",
    "source": "Source",
    "hierarchy": "Hierarchy",
    "viewInGraph": "View in Graph"
  },
  "compare": {
    "title": "Compare Frameworks",
    "selectFramework": "Select framework...",
    "mappings": "mappings"
  },
  "export": {
    "title": "Export Graph",
    "format": "Format",
    "size": "Size",
    "currentView": "Current view",
    "fullGraph": "Full graph",
    "include": "Include",
    "legend": "Legend",
    "exportTitle": "Title",
    "cancel": "Cancel",
    "export": "Export"
  },
  "status": {
    "selected": "Selected",
    "noSelection": "No selection",
    "connections": "connections"
  }
}
```

**Step 3: Update Norwegian translations**

```json
{
  "title": "Ontologi-utforsker",
  "frameworks": {
    "title": "Rammeverk",
    "iso31000": "ISO 31000:2018 - Risikostyring",
    "iso31010": "ISO 31010 - Risikovurderingsteknikker",
    "nistCsf": "NIST Cybersikkerhetsrammeverk"
  },
  "concepts": {
    "title": "Konsepter",
    "search": "Søk i alle konsepter...",
    "filter": "Filtrer...",
    "properties": "Egenskaper",
    "relationships": "Relasjoner",
    "noResults": "Ingen konsepter funnet"
  },
  "graph": {
    "title": "Konseptgraf",
    "zoomIn": "Zoom inn",
    "zoomOut": "Zoom ut",
    "resetView": "Tilbakestill visning",
    "fitToScreen": "Tilpass til skjerm",
    "toggleMinimap": "Vis/skjul minikart"
  },
  "detail": {
    "backToGraph": "Tilbake til graf",
    "definition": "Definisjon",
    "source": "Kilde",
    "hierarchy": "Hierarki",
    "viewInGraph": "Vis i graf"
  },
  "compare": {
    "title": "Sammenlign rammeverk",
    "selectFramework": "Velg rammeverk...",
    "mappings": "koblinger"
  },
  "export": {
    "title": "Eksporter graf",
    "format": "Format",
    "size": "Størrelse",
    "currentView": "Nåværende visning",
    "fullGraph": "Full graf",
    "include": "Inkluder",
    "legend": "Forklaring",
    "exportTitle": "Tittel",
    "cancel": "Avbryt",
    "export": "Eksporter"
  },
  "status": {
    "selected": "Valgt",
    "noSelection": "Ingen valgt",
    "connections": "koblinger"
  }
}
```

**Step 4: Verify TypeScript compiles**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 5: Commit**

```bash
git add frontend/src/routes/ontology/index.tsx
git add frontend/src/i18n/locales/en/ontology.json
git add frontend/src/i18n/locales/nb/ontology.json
git commit -m "feat(ontology): integrate OntologyExplorer into route and update translations"
```

---

## Task 13: Test and Verify

**Step 1: Run linting**

Run: `cd frontend && pnpm lint`
Expected: No errors (warnings acceptable)

**Step 2: Run type check**

Run: `cd frontend && pnpm typecheck`
Expected: No errors

**Step 3: Start dev server and test manually**

Run: `cd frontend && pnpm dev`
Expected: Server starts on port 5573

**Step 4: Test in browser**

1. Open http://localhost:5573/ontology
2. Verify sidebar loads with frameworks
3. Expand a framework and see concepts
4. Click a concept to select it
5. Verify graph renders with nodes and edges
6. Test zoom controls (+, -, 0)
7. Test view switching (Graph/Detail/Compare)
8. Test language toggle
9. Test search functionality

**Step 5: Final commit**

```bash
git add -A
git commit -m "feat(ontology): complete Ontology Explorer implementation

- Collapsible sidebar with framework tree navigation
- D3.js force-directed graph visualization
- Concept detail view with relationships and breadcrumb
- Framework comparison view with cross-mappings
- Graph controls: zoom, pan, fit, minimap
- Keyboard shortcuts for common actions
- PNG/SVG export functionality
- Full i18n support (EN/NB)

Closes #XX"
```

---

## Summary

| Task | Description | Files |
|------|-------------|-------|
| 1 | Install dependencies | package.json |
| 2 | Create TypeScript types | types/index.ts |
| 3 | Create API hooks | api/index.ts |
| 4 | Create utility functions | utils/*.ts |
| 5 | Create explorer context | context/*.tsx |
| 6 | Create sidebar components | Sidebar/*.tsx |
| 7 | Create graph components | Graph/*.tsx, hooks/useD3Graph.ts |
| 8 | Create detail view | Detail/*.tsx |
| 9 | Create compare view | Compare/*.tsx |
| 10 | Create export dialog | ExportDialog.tsx |
| 11 | Create main explorer | OntologyExplorer.tsx |
| 12 | Update route and i18n | routes/ontology/index.tsx, locales/*.json |
| 13 | Test and verify | - |
