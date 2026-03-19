import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { SettingsForm } from "../SettingsForm";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const mockMutate = vi.fn();

vi.mock("@/features/analysis/api", () => ({
  usePromptTemplate: vi.fn().mockReturnValue({
    data: {
      version: 1,
      thresholds: { min_confidence: 0.5, addressed: 0.8, partial: 0.6 },
      max_findings_per_framework: 100,
      include_addressed_findings: true,
      boost_terms: { security: 1.5, compliance: 2.0 },
    },
    isLoading: false,
    isError: false,
  }),
  useUpdatePromptTemplate: () => ({
    mutate: mockMutate,
    isPending: false,
    isSuccess: false,
  }),
}));

function renderWithQuery(ui: React.ReactElement) {
  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  return render(
    React.createElement(QueryClientProvider, { client: queryClient }, ui)
  );
}

describe("SettingsForm", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders all threshold inputs with current values", () => {
    renderWithQuery(<SettingsForm />);
    expect(screen.getByDisplayValue("0.5")).toBeDefined(); // min_confidence
    expect(screen.getByDisplayValue("0.8")).toBeDefined(); // addressed
    expect(screen.getByDisplayValue("0.6")).toBeDefined(); // partial
    expect(screen.getByDisplayValue("100")).toBeDefined(); // max_findings
    expect(screen.getByText("settings.includeAddressed")).toBeDefined();
  });

  it("renders boost terms list with add/remove", () => {
    renderWithQuery(<SettingsForm />);
    expect(screen.getByDisplayValue("security")).toBeDefined();
    expect(screen.getByDisplayValue("compliance")).toBeDefined();
    expect(screen.getByText("settings.addTerm")).toBeDefined();
  });

  it("save button calls updatePromptTemplate mutation", async () => {
    renderWithQuery(<SettingsForm />);
    const saveBtn = screen.getByText("settings.save");
    fireEvent.click(saveBtn);
    await waitFor(() => {
      expect(mockMutate).toHaveBeenCalledWith(
        expect.objectContaining({
          version: 1,
          thresholds: expect.objectContaining({ min_confidence: 0.5 }),
        })
      );
    });
  });

  it("reset button restores default values without saving", async () => {
    // Mock window.confirm
    vi.spyOn(window, "confirm").mockReturnValue(true);

    renderWithQuery(<SettingsForm />);
    const resetBtn = screen.getByText("settings.resetDefaults");
    fireEvent.click(resetBtn);

    // Default values should now be shown
    await waitFor(() => {
      expect(screen.getByDisplayValue("0.1")).toBeDefined(); // default min_confidence
    });
    expect(screen.getByDisplayValue("50")).toBeDefined(); // default max_findings

    // Mutation should NOT have been called
    expect(mockMutate).not.toHaveBeenCalled();

    vi.restoreAllMocks();
  });
});
