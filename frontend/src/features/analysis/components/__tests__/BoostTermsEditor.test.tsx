import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { BoostTermsEditor } from "../BoostTermsEditor";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

describe("BoostTermsEditor", () => {
  it("renders existing terms as rows", () => {
    const onChange = vi.fn();
    render(
      <BoostTermsEditor
        value={[
          { term: "risk", weight: 1.0 },
          { term: "threat", weight: 2.0 },
        ]}
        onChange={onChange}
      />
    );
    const inputs = screen.getAllByDisplayValue("risk");
    expect(inputs).toHaveLength(1);
    expect(screen.getByDisplayValue("2")).toBeDefined();
  });

  it("add button creates new empty row", () => {
    const onChange = vi.fn();
    render(
      <BoostTermsEditor value={[]} onChange={onChange} />
    );
    const addBtn = screen.getByText("settings.addTerm");
    fireEvent.click(addBtn);
    expect(onChange).toHaveBeenCalledWith([{ term: "", weight: 1.0 }]);
  });

  it("delete button removes a row", () => {
    const onChange = vi.fn();
    render(
      <BoostTermsEditor
        value={[
          { term: "risk", weight: 1.0 },
          { term: "threat", weight: 2.0 },
        ]}
        onChange={onChange}
      />
    );
    const deleteButtons = screen.getAllByRole("button").filter(
      (btn) => !btn.textContent?.includes("settings.addTerm")
    );
    fireEvent.click(deleteButtons[0]);
    expect(onChange).toHaveBeenCalledWith([{ term: "threat", weight: 2.0 }]);
  });

  it("empty term input gets aria-invalid attribute", () => {
    render(
      <BoostTermsEditor
        value={[{ term: "", weight: 1.0 }]}
        onChange={vi.fn()}
      />
    );
    const termInputs = screen.getAllByPlaceholderText("settings.termLabel");
    expect(termInputs[0].getAttribute("aria-invalid")).toBe("true");
  });
});
