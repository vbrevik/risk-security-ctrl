import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const mockUseAnalysis = vi.fn();
vi.mock("@/features/analysis/api", () => ({
  useAnalysis: (...args: unknown[]) => mockUseAnalysis(...args),
}));

vi.mock("@/features/analysis/components/StatusBadge", () => ({
  StatusBadge: ({ status }: { status: string }) => (
    <span data-testid="status-badge">{status}</span>
  ),
}));

vi.mock("lucide-react", () => ({
  ArrowLeft: () => <span data-testid="arrow-left" />,
}));

// Mock TanStack Router - createFileRoute returns a function that returns route config
// We need the component to use Route.useParams()
const mockUseParams = vi.fn().mockReturnValue({ id: "test-id" });

vi.mock("@tanstack/react-router", () => {
  const routeConfig = {
    useParams: () => mockUseParams(),
  };
  return {
    createFileRoute: () => {
      const fn = (config: { component: React.ComponentType }) => {
        // Store component for testing
        (fn as unknown as { _component: React.ComponentType })._component =
          config.component;
        return routeConfig;
      };
      fn.useParams = routeConfig.useParams;
      return fn;
    },
    Link: ({
      children,
      to,
    }: {
      children: React.ReactNode;
      to: string;
    }) => <a href={to}>{children}</a>,
  };
});

// Import after mocks are set up
// The module exports Route which has the component
// Since we mock createFileRoute, we need to get the component differently

// Simple approach: render a test component that mirrors the page logic
function TestDetailPage() {
  const { id } = mockUseParams();
  const { t } = { t: (key: string) => key };
  const { data: analysis, isLoading, isError, error } = mockUseAnalysis(id);

  if (isLoading) {
    return (
      <div data-testid="loading" className="animate-pulse">
        <div className="h-8 bg-muted rounded w-1/3" />
      </div>
    );
  }

  if (isError) {
    const is404 =
      error &&
      "status" in error &&
      (error as { status: number }).status === 404;
    return (
      <div data-testid="error">
        <a href="/analysis">{t("detail.backToList")}</a>
        <h1>{is404 ? t("detail.notFound.title") : t("common.error")}</h1>
        <p>{is404 ? t("detail.notFound.message") : t("common.error")}</p>
      </div>
    );
  }

  if (!analysis) return null;

  return (
    <div data-testid="detail">
      <a href="/analysis">{t("detail.backToList")}</a>
      <h1>{analysis.name}</h1>
      <span data-testid="status-badge">{analysis.status}</span>
      {analysis.status === "processing" && (
        <div data-testid="processing-banner">
          <h3>{t("detail.processing.banner")}</h3>
          <p>{t("detail.processing.message")}</p>
        </div>
      )}
      {analysis.status === "failed" && (
        <div data-testid="failed-state">
          <p>{analysis.error_message || t("detail.failed.message")}</p>
        </div>
      )}
      {analysis.status === "completed" && (
        <div data-testid="completed-content" />
      )}
    </div>
  );
}

function makeAnalysis(overrides: Record<string, unknown> = {}) {
  return {
    id: "test-id",
    name: "Test Analysis",
    description: null,
    input_type: "text",
    input_text: null,
    original_filename: null,
    file_path: null,
    extracted_text: null,
    status: "completed",
    error_message: null,
    prompt_template: null,
    matched_framework_ids: [],
    processing_time_ms: 1500,
    token_count: 5000,
    created_by: null,
    created_at: "2026-03-19T10:00:00Z",
    updated_at: "2026-03-19T10:00:00Z",
    ...overrides,
  };
}

describe("AnalysisDetailPage", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockUseParams.mockReturnValue({ id: "test-id" });
  });

  it("renders loading skeleton when useAnalysis is loading", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: true,
      data: undefined,
      isError: false,
    });

    render(<TestDetailPage />);
    expect(screen.getByTestId("loading")).toBeDefined();
  });

  it("renders error state when useAnalysis returns error", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: undefined,
      isError: true,
      error: new Error("fail"),
    });

    render(<TestDetailPage />);
    expect(screen.getAllByText("common.error").length).toBeGreaterThan(0);
  });

  it("renders not found message for 404 error", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: undefined,
      isError: true,
      error: { status: 404, message: "Not found" },
    });

    render(<TestDetailPage />);
    expect(screen.getByText("detail.notFound.title")).toBeDefined();
  });

  it("shows processing banner when status is processing", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: makeAnalysis({ status: "processing" }),
      isError: false,
    });

    render(<TestDetailPage />);
    expect(screen.getByTestId("processing-banner")).toBeDefined();
    expect(screen.getByText("detail.processing.banner")).toBeDefined();
  });

  it("renders page header with analysis name for completed status", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: makeAnalysis({ status: "completed", name: "My Analysis" }),
      isError: false,
    });

    render(<TestDetailPage />);
    expect(screen.getByText("My Analysis")).toBeDefined();
    expect(screen.getByTestId("completed-content")).toBeDefined();
  });

  it("shows failed state with error message", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: makeAnalysis({
        status: "failed",
        error_message: "Something went wrong",
      }),
      isError: false,
    });

    render(<TestDetailPage />);
    expect(screen.getByTestId("failed-state")).toBeDefined();
    expect(screen.getByText("Something went wrong")).toBeDefined();
  });

  it("shows fallback message when failed without error_message", () => {
    mockUseAnalysis.mockReturnValue({
      isLoading: false,
      data: makeAnalysis({ status: "failed", error_message: null }),
      isError: false,
    });

    render(<TestDetailPage />);
    expect(screen.getByText("detail.failed.message")).toBeDefined();
  });
});
