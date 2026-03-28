import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import React from "react";
import { VerificationBadge } from "../VerificationBadge";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string, fallback?: string) => fallback ?? key,
  }),
}));

describe("VerificationBadge", () => {
  it('renders "Verified" label for status="verified"', () => {
    render(<VerificationBadge status="verified" />);
    expect(screen.getByText("Verified")).toBeInTheDocument();
  });

  it('renders correct label for status="partially-verified"', () => {
    render(<VerificationBadge status="partially-verified" />);
    expect(screen.getByText("Partially Verified")).toBeInTheDocument();
  });

  it('renders correct label for status="structure-verified"', () => {
    render(<VerificationBadge status="structure-verified" />);
    expect(screen.getByText("Structure Verified")).toBeInTheDocument();
  });

  it('renders correct label for status="unverified"', () => {
    render(<VerificationBadge status="unverified" />);
    expect(screen.getByText("Unverified")).toBeInTheDocument();
  });

  it('renders correct label for status="needs-correction"', () => {
    render(<VerificationBadge status="needs-correction" />);
    expect(screen.getByText("Needs Correction")).toBeInTheDocument();
  });

  it('renders correct label for status="corrected"', () => {
    render(<VerificationBadge status="corrected" />);
    expect(screen.getByText("Corrected")).toBeInTheDocument();
  });

  it("renders without crashing when status is null (fallback style)", () => {
    const { container } = render(<VerificationBadge status={null} />);
    expect(container.firstChild).not.toBeNull();
    expect(screen.getByText("Unknown")).toBeInTheDocument();
  });

  it("renders without crashing when status is an unknown string", () => {
    const { container } = render(<VerificationBadge status="banana" />);
    expect(container.firstChild).not.toBeNull();
    expect(screen.getByText("Unknown")).toBeInTheDocument();
  });

  it("rendered element has aria-label attribute", () => {
    const { container } = render(<VerificationBadge status="verified" />);
    const badge = container.querySelector("[aria-label]");
    expect(badge).not.toBeNull();
    expect(badge?.getAttribute("aria-label")).toBeTruthy();
  });
});
