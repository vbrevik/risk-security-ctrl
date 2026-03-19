import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { FileDropZone } from "../FileDropZone";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const defaultProps = {
  accept: [".pdf", ".docx"],
  maxSizeMB: 25,
  selectedFile: null,
  onFileSelected: vi.fn(),
  onClear: vi.fn(),
  onError: vi.fn(),
};

describe("FileDropZone", () => {
  it("renders drop zone with browse link", () => {
    render(<FileDropZone {...defaultProps} />);
    expect(screen.getByText("create.dropzoneText")).toBeDefined();
    expect(screen.getByText("create.dropzoneBrowse")).toBeDefined();
  });

  it("accepts PDF files via input change", () => {
    const onFileSelected = vi.fn();
    render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
    fireEvent.change(input, { target: { files: [file] } });
    expect(onFileSelected).toHaveBeenCalledWith(file);
  });

  it("accepts DOCX files via input change", () => {
    const onFileSelected = vi.fn();
    render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    const file = new File(["content"], "test.docx", {
      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    });
    fireEvent.change(input, { target: { files: [file] } });
    expect(onFileSelected).toHaveBeenCalledWith(file);
  });

  it("rejects files exceeding maxSizeMB", () => {
    const onError = vi.fn();
    const onFileSelected = vi.fn();
    render(<FileDropZone {...defaultProps} onError={onError} onFileSelected={onFileSelected} />);
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    const bigFile = new File(["x".repeat(26 * 1024 * 1024)], "big.pdf", { type: "application/pdf" });
    Object.defineProperty(bigFile, "size", { value: 26 * 1024 * 1024 });
    fireEvent.change(input, { target: { files: [bigFile] } });
    expect(onError).toHaveBeenCalled();
    expect(onFileSelected).not.toHaveBeenCalled();
  });

  it("rejects non-PDF/DOCX files", () => {
    const onError = vi.fn();
    const onFileSelected = vi.fn();
    render(<FileDropZone {...defaultProps} onError={onError} onFileSelected={onFileSelected} />);
    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
    const txtFile = new File(["content"], "test.txt", { type: "text/plain" });
    fireEvent.change(input, { target: { files: [txtFile] } });
    expect(onError).toHaveBeenCalled();
    expect(onFileSelected).not.toHaveBeenCalled();
  });

  it("shows drag highlight on dragEnter, removes on dragLeave", () => {
    const { container } = render(<FileDropZone {...defaultProps} />);
    const dropZone = container.firstElementChild as HTMLElement;
    fireEvent.dragEnter(dropZone, { dataTransfer: { items: [] } });
    expect(dropZone.className).toMatch(/border-primary/);
    fireEvent.dragLeave(dropZone);
    expect(dropZone.className).not.toMatch(/border-primary/);
  });

  it("calls onFileSelected with dropped file", () => {
    const onFileSelected = vi.fn();
    const { container } = render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
    const dropZone = container.firstElementChild as HTMLElement;
    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
    fireEvent.drop(dropZone, {
      dataTransfer: { files: [file] },
    });
    expect(onFileSelected).toHaveBeenCalledWith(file);
  });
});
