import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import { StatusBadge } from "../StatusBadge";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

describe("StatusBadge", () => {
  it("renders green badge for completed status", () => {
    const { container } = render(<StatusBadge status="completed" />);
    expect(screen.getByText("status.completed")).toBeDefined();
    expect(container.innerHTML).toMatch(/bg-green/);
  });

  it("renders yellow badge with pulse for processing status", () => {
    const { container } = render(<StatusBadge status="processing" />);
    expect(screen.getByText("status.processing")).toBeDefined();
    expect(container.innerHTML).toMatch(/animate-pulse/);
  });

  it("renders red badge for failed status", () => {
    const { container } = render(<StatusBadge status="failed" />);
    expect(screen.getByText("status.failed")).toBeDefined();
    expect(container.innerHTML).toMatch(/bg-red/);
  });

  it("renders blue badge for pending status", () => {
    const { container } = render(<StatusBadge status="pending" />);
    expect(screen.getByText("status.pending")).toBeDefined();
    expect(container.innerHTML).toMatch(/bg-blue/);
  });
});
