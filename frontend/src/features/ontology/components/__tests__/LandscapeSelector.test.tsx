import { describe, it, expect, vi } from "vitest";
import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { LandscapeSelector } from "../LandscapeSelector";

describe("LandscapeSelector", () => {
  it("renders sector radio buttons", () => {
    render(
      <LandscapeSelector
        sector={undefined}
        activities={[]}
        onSectorChange={vi.fn()}
        onActivitiesChange={vi.fn()}
      />
    );
    expect(screen.getByText("Financial")).toBeInTheDocument();
    expect(screen.getByText("Healthcare")).toBeInTheDocument();
    expect(screen.getByText("Critical Infrastructure")).toBeInTheDocument();
    expect(screen.getByText("Government/Public Admin")).toBeInTheDocument();
    expect(screen.getByText("Technology/AI Provider")).toBeInTheDocument();
    expect(screen.getByText("General Enterprise")).toBeInTheDocument();
  });

  it("renders activity checkboxes", () => {
    render(
      <LandscapeSelector
        sector={undefined}
        activities={[]}
        onSectorChange={vi.fn()}
        onActivitiesChange={vi.fn()}
      />
    );
    expect(screen.getByText("Processing personal data")).toBeInTheDocument();
    expect(screen.getByText("Deploying AI systems")).toBeInTheDocument();
    expect(screen.getByText("Defense/NATO context")).toBeInTheDocument();
  });

  it("sector selection calls callback", async () => {
    const onSectorChange = vi.fn();
    const user = userEvent.setup();
    render(
      <LandscapeSelector
        sector={undefined}
        activities={[]}
        onSectorChange={onSectorChange}
        onActivitiesChange={vi.fn()}
      />
    );
    await user.click(screen.getByText("Financial"));
    expect(onSectorChange).toHaveBeenCalledWith("Financial");
  });

  it("toggling activity calls callback", async () => {
    const onActivitiesChange = vi.fn();
    const user = userEvent.setup();
    render(
      <LandscapeSelector
        sector={undefined}
        activities={[]}
        onSectorChange={vi.fn()}
        onActivitiesChange={onActivitiesChange}
      />
    );
    await user.click(screen.getByText("Deploying AI systems"));
    expect(onActivitiesChange).toHaveBeenCalledWith(["Deploying AI systems"]);
  });

  it("clear all resets activities", async () => {
    const onActivitiesChange = vi.fn();
    const user = userEvent.setup();
    render(
      <LandscapeSelector
        sector={undefined}
        activities={["Deploying AI systems", "Financial services"]}
        onSectorChange={vi.fn()}
        onActivitiesChange={onActivitiesChange}
      />
    );
    await user.click(screen.getByText("Clear all"));
    expect(onActivitiesChange).toHaveBeenCalledWith([]);
  });
});
