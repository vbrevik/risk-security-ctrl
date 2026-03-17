import { createContext, useCallback, useContext, useMemo, useReducer, type ReactNode } from "react";
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
  | { type: "SET_SEARCH_HIGHLIGHTS"; ids: string[] }
  | { type: "NAVIGATE_BACK"; conceptId: string }
  | { type: "SET_ACTIVE_TOPICS"; topicIds: string[] }
  | { type: "TOGGLE_TOPIC"; topicId: string };

const initialState: ExplorerState = {
  selectedConceptId: null,
  selectedConcepts: [],
  viewMode: "graph",
  sidebarCollapsed: false,
  compareFrameworks: [null, null],
  activeFrameworks: [],
  activeTopics: [],
  activeConceptType: null,
  searchHighlightIds: [],
  navigationHistory: [],
};

function explorerReducer(state: ExplorerState, action: ExplorerAction): ExplorerState {
  switch (action.type) {
    case "SELECT_CONCEPT": {
      const newHistory = action.conceptId
        ? [...state.navigationHistory, action.conceptId]
        : state.navigationHistory;
      return {
        ...state,
        selectedConceptId: action.conceptId,
        selectedConcepts: action.conceptId ? [action.conceptId] : [],
        navigationHistory: newHistory,
      };
    }
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
        navigationHistory: [],
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
    case "NAVIGATE_BACK": {
      const idx = state.navigationHistory.lastIndexOf(action.conceptId);
      const truncated = idx >= 0
        ? state.navigationHistory.slice(0, idx + 1)
        : state.navigationHistory;
      return {
        ...state,
        selectedConceptId: action.conceptId,
        selectedConcepts: [action.conceptId],
        navigationHistory: truncated,
      };
    }
    case "SET_ACTIVE_TOPICS":
      return { ...state, activeTopics: action.topicIds };
    case "TOGGLE_TOPIC": {
      const isActive = state.activeTopics.includes(action.topicId);
      return {
        ...state,
        activeTopics: isActive
          ? state.activeTopics.filter((id) => id !== action.topicId)
          : [...state.activeTopics, action.topicId],
      };
    }
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
  navigateBack: (conceptId: string) => void;
  setActiveTopics: (topicIds: string[]) => void;
  toggleTopic: (topicId: string) => void;
}

const ExplorerContext = createContext<ExplorerContextValue | null>(null);

export function ExplorerProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(explorerReducer, initialState);

  // Stabilize all dispatch wrappers so consumers don't re-render on every provider render
  const selectConcept = useCallback((conceptId: string | null) => dispatch({ type: "SELECT_CONCEPT", conceptId }), []);
  const toggleConceptSelection = useCallback((conceptId: string) => dispatch({ type: "TOGGLE_CONCEPT", conceptId }), []);
  const clearSelection = useCallback(() => dispatch({ type: "CLEAR_SELECTION" }), []);
  const setViewMode = useCallback((mode: ViewMode) => dispatch({ type: "SET_VIEW_MODE", mode }), []);
  const toggleSidebar = useCallback(() => dispatch({ type: "TOGGLE_SIDEBAR" }), []);
  const setCompareLeft = useCallback((frameworkId: string | null) => dispatch({ type: "SET_COMPARE_LEFT", frameworkId }), []);
  const setCompareRight = useCallback((frameworkId: string | null) => dispatch({ type: "SET_COMPARE_RIGHT", frameworkId }), []);
  const setActiveFrameworks = useCallback((frameworkIds: string[]) => dispatch({ type: "SET_ACTIVE_FRAMEWORKS", frameworkIds }), []);
  const toggleFramework = useCallback((frameworkId: string) => dispatch({ type: "TOGGLE_FRAMEWORK", frameworkId }), []);
  const setConceptType = useCallback((conceptType: string | null) => dispatch({ type: "SET_CONCEPT_TYPE", conceptType }), []);
  const setSearchHighlights = useCallback((ids: string[]) => dispatch({ type: "SET_SEARCH_HIGHLIGHTS", ids }), []);
  const navigateBack = useCallback((conceptId: string) => dispatch({ type: "NAVIGATE_BACK", conceptId }), []);
  const setActiveTopics = useCallback((topicIds: string[]) => dispatch({ type: "SET_ACTIVE_TOPICS", topicIds }), []);
  const toggleTopic = useCallback((topicId: string) => dispatch({ type: "TOGGLE_TOPIC", topicId }), []);

  const value = useMemo<ExplorerContextValue>(() => ({
    state,
    selectConcept,
    toggleConceptSelection,
    clearSelection,
    setViewMode,
    toggleSidebar,
    setCompareLeft,
    setCompareRight,
    setActiveFrameworks,
    toggleFramework,
    setConceptType,
    setSearchHighlights,
    navigateBack,
    setActiveTopics,
    toggleTopic,
  }), [
    state,
    selectConcept,
    toggleConceptSelection,
    clearSelection,
    setViewMode,
    toggleSidebar,
    setCompareLeft,
    setCompareRight,
    setActiveFrameworks,
    toggleFramework,
    setConceptType,
    setSearchHighlights,
    navigateBack,
    setActiveTopics,
    toggleTopic,
  ]);

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
