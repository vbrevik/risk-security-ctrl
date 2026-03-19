import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent, waitFor } from "@testing-library/react";
import { ExportButtons } from "../ExportButtons";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const mockExportAnalysis = vi.fn();
vi.mock("../../api", () => ({
  exportAnalysis: (...args: unknown[]) => mockExportAnalysis(...args),
}));

describe("ExportButtons", () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockExportAnalysis.mockResolvedValue(undefined);
  });

  it("renders PDF and DOCX export buttons", () => {
    render(
      <ExportButtons
        analysisId="a1"
        analysisName="Test"
        status="completed"
      />
    );
    expect(screen.getByText("export.pdf")).toBeInTheDocument();
    expect(screen.getByText("export.docx")).toBeInTheDocument();
  });

  it("buttons disabled when status is not completed", () => {
    render(
      <ExportButtons
        analysisId="a1"
        analysisName="Test"
        status="processing"
      />
    );
    const buttons = screen.getAllByRole("button");
    buttons.forEach((btn) => expect(btn).toBeDisabled());
  });

  it("disabled buttons show tooltip via title attribute", () => {
    render(
      <ExportButtons
        analysisId="a1"
        analysisName="Test"
        status="processing"
      />
    );
    const buttons = screen.getAllByRole("button");
    buttons.forEach((btn) =>
      expect(btn.getAttribute("title")).toBe("export.disabled")
    );
  });

  it("clicking PDF button calls exportAnalysis with pdf format", async () => {
    render(
      <ExportButtons
        analysisId="a1"
        analysisName="My Analysis"
        status="completed"
      />
    );
    fireEvent.click(screen.getByText("export.pdf"));
    await waitFor(() => {
      expect(mockExportAnalysis).toHaveBeenCalledWith("a1", "pdf", "My Analysis");
    });
  });

  it("clicking DOCX button calls exportAnalysis with docx format", async () => {
    render(
      <ExportButtons
        analysisId="a1"
        analysisName="My Analysis"
        status="completed"
      />
    );
    fireEvent.click(screen.getByText("export.docx"));
    await waitFor(() => {
      expect(mockExportAnalysis).toHaveBeenCalledWith(
        "a1",
        "docx",
        "My Analysis"
      );
    });
  });

  it("both buttons disabled while export is in progress", async () => {
    let resolveExport: () => void;
    mockExportAnalysis.mockImplementation(
      () => new Promise<void>((r) => (resolveExport = r))
    );

    render(
      <ExportButtons
        analysisId="a1"
        analysisName="Test"
        status="completed"
      />
    );

    fireEvent.click(screen.getByText("export.pdf"));

    await waitFor(() => {
      const buttons = screen.getAllByRole("button");
      buttons.forEach((btn) => expect(btn).toBeDisabled());
    });

    // Resolve to clean up
    resolveExport!();
  });
});
