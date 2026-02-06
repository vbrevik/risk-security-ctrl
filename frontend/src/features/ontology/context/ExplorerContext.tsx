import { createContext, useContext, useReducer, type ReactNode } from "react";
import type { ViewMode, ExplorerState } from "../types";

type ExplorerAction =
  | { type: "SELECT_CONCEPT"; conceptId: string | null }
  | { type: "TOGGLE_CONCEPT"; conceptId: string }
  | { type: "CLEAR_SELECTION" }
  | { type: "SET_VIEW_MODE"; mode: ViewMode }
  | { type: "TOGGLE_SIDEBAR" }
  | { type: "SET_COMPARE_LEFT"; frameworkId: string | null }
  | { type: "SET_COMPARE_RIGHT"; frameworkId: string | null }
  | { type: "SET_ACTIVE_FRAMEWORKS"; frameworkIds: string[] }
  | { type: "TOGGLE_FRAMEWORK"; frameworkId: string }
  | { type: "SET_CONCEPT_TYPE"; conceptType: string | null }
  | { type: "SET_SEARCH_HIGHLIGHTS"; ids: string[] };

const initialState: ExplorerState = {
  selectedConceptId: null,
  selectedConcepts: [],
  viewMode: "graph",
  sidebarCollapsed: false,
  compareFrameworks: [null, null],
  activeFrameworks: [],
  activeConceptType: null,
  searchHighlightIds: [],
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
    case "SET_ACTIVE_FRAMEWORKS":
      return { ...state, activeFrameworks: action.frameworkIds };
    case "TOGGLE_FRAMEWORK": {
      const isActive = state.activeFrameworks.includes(action.frameworkId);
      return {
        ...state,
        activeFrameworks: isActive
          ? state.activeFrameworks.filter((id) => id !== action.frameworkId)
          : [...state.activeFrameworks, action.frameworkId],
      };
    }
    case "SET_CONCEPT_TYPE":
      return { ...state, activeConceptType: action.conceptType };
    case "SET_SEARCH_HIGHLIGHTS":
      return { ...state, searchHighlightIds: action.ids };
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
  setActiveFrameworks: (frameworkIds: string[]) => void;
  toggleFramework: (frameworkId: string) => void;
  setConceptType: (conceptType: string | null) => void;
  setSearchHighlights: (ids: string[]) => void;
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
    setActiveFrameworks: (frameworkIds) => dispatch({ type: "SET_ACTIVE_FRAMEWORKS", frameworkIds }),
    toggleFramework: (frameworkId) => dispatch({ type: "TOGGLE_FRAMEWORK", frameworkId }),
    setConceptType: (conceptType) => dispatch({ type: "SET_CONCEPT_TYPE", conceptType }),
    setSearchHighlights: (ids) => dispatch({ type: "SET_SEARCH_HIGHLIGHTS", ids }),
  };

  return (
    <ExplorerContext.Provider value={value}>{children}</ExplorerContext.Provider>
  );
}

// eslint-disable-next-line react-refresh/only-export-components
export function useExplorer() {
  const context = useContext(ExplorerContext);
  if (!context) {
    throw new Error("useExplorer must be used within an ExplorerProvider");
  }
  return context;
}
