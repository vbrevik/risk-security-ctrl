import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { EmptyFindings } from "../EmptyFindings";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

vi.mock("@tanstack/react-router", () => ({
  Link: ({
    children,
    to,
  }: {
    children: React.ReactNode;
    to: string;
  }) => <a href={to}>{children}</a>,
}));

describe("EmptyFindings", () => {
  it("renders no compliance findings heading", () => {
    render(<EmptyFindings />);
    expect(screen.getByText("findings.empty.title")).toBeInTheDocument();
  });

  it("renders suggestion text", () => {
    render(<EmptyFindings />);
    expect(
      screen.getByText("findings.empty.description")
    ).toBeInTheDocument();
  });

  it("renders link to /analysis/settings", () => {
    render(<EmptyFindings />);
    const link = screen.getByText("findings.empty.settingsLink");
    expect(link.closest("a")?.getAttribute("href")).toBe(
      "/analysis/settings"
    );
  });
});
