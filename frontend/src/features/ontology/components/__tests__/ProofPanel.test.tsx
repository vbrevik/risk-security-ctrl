import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { ProofPanel } from "../ProofPanel";
import type { FrameworkProof } from "../../types";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, fallback?: string) => fallback ?? key,
  }),
}));

vi.mock("../../api", () => ({
  useFrameworkProof: vi.fn(),
}));

vi.mock("../VerificationBadge", () => ({
  VerificationBadge: ({ status }: { status: string | null }) => (
    <div data-testid="verification-badge">{status ?? "unknown"}</div>
  ),
}));

import { useFrameworkProof } from "../../api";
const mockedHook = vi.mocked(useFrameworkProof);

function createWrapper() {
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(
      QueryClientProvider,
      { client: queryClient },
      children
    );
  };
}

function makeProof(overrides: Partial<FrameworkProof> = {}): FrameworkProof {
  return {
    framework_id: "nist-csf",
    verification_status: "verified",
    verification_date: "2025-01-15",
    verification_source: null,
    verification_notes: null,
    proof_content: null,
    ...overrides,
  };
}

describe("ProofPanel", () => {
  beforeEach(() => {
    vi.resetAllMocks();
  });

  it("renders skeleton elements while loading", () => {
    mockedHook.mockReturnValue({
      isLoading: true,
      isError: false,
      data: undefined,
    } as ReturnType<typeof useFrameworkProof>);

    const { container } = render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    const skeletons = container.querySelectorAll(".animate-pulse");
    expect(skeletons.length).toBeGreaterThanOrEqual(3);
  });

  it("renders generic error message without internal paths on error", () => {
    mockedHook.mockReturnValue({
      isLoading: false,
      isError: true,
      data: undefined,
    } as ReturnType<typeof useFrameworkProof>);

    const { container } = render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    // Should show the safe generic error message
    expect(screen.getByText("Could not load proof document.")).toBeInTheDocument();
    // Should NOT expose internal paths or API details
    expect(container.innerHTML).not.toMatch(/docs\/sources/);
    expect(container.innerHTML).not.toMatch(/api\/ontology/);
  });

  it("renders markdown content heading when proof_content is non-null", async () => {
    mockedHook.mockReturnValue({
      isLoading: false,
      isError: false,
      data: makeProof({ proof_content: "# Verification\n\nSome text here." }),
    } as ReturnType<typeof useFrameworkProof>);

    render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    expect(screen.getByRole("heading", { name: /verification/i })).toBeInTheDocument();
  });

  it("renders no-proof message when proof_content is null", () => {
    mockedHook.mockReturnValue({
      isLoading: false,
      isError: false,
      data: makeProof({ proof_content: null }),
    } as ReturnType<typeof useFrameworkProof>);

    render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    expect(
      screen.getByText("No proof document available")
    ).toBeInTheDocument();
  });

  it("source link has rel=noopener noreferrer when verification_source is present", () => {
    mockedHook.mockReturnValue({
      isLoading: false,
      isError: false,
      data: makeProof({
        verification_source: "https://example.com/nist-csf-proof",
      }),
    } as ReturnType<typeof useFrameworkProof>);

    render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    const link = screen.getByRole("link");
    expect(link).toHaveAttribute("rel", "noopener noreferrer");
    expect(link).toHaveAttribute("target", "_blank");
  });

  it("no external link rendered when verification_source is null", () => {
    mockedHook.mockReturnValue({
      isLoading: false,
      isError: false,
      data: makeProof({ verification_source: null }),
    } as ReturnType<typeof useFrameworkProof>);

    render(
      <ProofPanel frameworkId="nist-csf" />,
      { wrapper: createWrapper() }
    );

    expect(screen.queryByRole("link")).toBeNull();
  });
});
