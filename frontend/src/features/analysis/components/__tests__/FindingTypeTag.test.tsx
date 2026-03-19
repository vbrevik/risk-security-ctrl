import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { FindingTypeTag } from "../FindingTypeTag";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

describe("FindingTypeTag", () => {
  it("renders green badge for addressed type", () => {
    const { container } = render(<FindingTypeTag type="addressed" />);
    expect(container.innerHTML).toMatch(/bg-green/);
    expect(screen.getByText("findings.type.addressed")).toBeInTheDocument();
  });

  it("renders yellow badge for partially_addressed type", () => {
    const { container } = render(<FindingTypeTag type="partially_addressed" />);
    expect(container.innerHTML).toMatch(/bg-yellow/);
    expect(
      screen.getByText("findings.type.partially_addressed")
    ).toBeInTheDocument();
  });

  it("renders red badge for gap type", () => {
    const { container } = render(<FindingTypeTag type="gap" />);
    expect(container.innerHTML).toMatch(/bg-red/);
    expect(screen.getByText("findings.type.gap")).toBeInTheDocument();
  });

  it("renders gray badge for not_applicable type", () => {
    const { container } = render(<FindingTypeTag type="not_applicable" />);
    expect(container.innerHTML).toMatch(/bg-gray/);
    expect(
      screen.getByText("findings.type.not_applicable")
    ).toBeInTheDocument();
  });
});
