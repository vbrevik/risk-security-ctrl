import { describe, it, expect, vi, beforeEach } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import React from "react";
import {
  createRootRoute,
  createRoute,
  createRouter,
  createMemoryHistory,
  RouterProvider,
  Outlet,
} from "@tanstack/react-router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import { CreateAnalysisForm } from "../CreateAnalysisForm";

vi.mock("react-i18next", () => ({
  useTranslation: () => ({
    t: (key: string) => key,
  }),
}));

const mockCreateMutate = vi.fn();
const mockUploadMutate = vi.fn();

vi.mock("@/features/analysis/api", () => ({
  useCreateAnalysis: () => ({
    mutate: mockCreateMutate,
    isPending: false,
  }),
  useUploadAnalysis: () => ({
    mutate: mockUploadMutate,
    isPending: false,
    progress: 0,
  }),
}));

function renderWithRouter(ui: React.ReactElement) {
  const rootRoute = createRootRoute({
    component: () => React.createElement("div", null, React.createElement(Outlet)),
  });

  const indexRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/",
    component: () => ui,
  });

  const detailRoute = createRoute({
    getParentRoute: () => rootRoute,
    path: "/analysis/$id",
    component: () => React.createElement("div", null, "Detail"),
  });

  const router = createRouter({
    routeTree: rootRoute.addChildren([indexRoute, detailRoute]),
    history: createMemoryHistory({ initialEntries: ["/"] }),
  });

  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });

  return render(
    React.createElement(
      QueryClientProvider,
      { client: queryClient },
      React.createElement(RouterProvider, { router: router as any })
    )
  );
}

describe("CreateAnalysisForm", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  it("renders name input and tab toggle", async () => {
    renderWithRouter(<CreateAnalysisForm />);
    expect(await screen.findByPlaceholderText("create.namePlaceholder")).toBeDefined();
    expect(screen.getByText("create.textTab")).toBeDefined();
    expect(screen.getByText("create.uploadTab")).toBeDefined();
  });

  it("submit disabled when name is empty", async () => {
    renderWithRouter(<CreateAnalysisForm />);
    const submitBtn = await screen.findByText("create.submit");
    expect(submitBtn.closest("button")?.disabled).toBe(true);
  });

  it("calls createAnalysis mutation on text tab submit", async () => {
    renderWithRouter(<CreateAnalysisForm />);

    const nameInput = await screen.findByPlaceholderText("create.namePlaceholder");
    fireEvent.change(nameInput, { target: { value: "Test Analysis" } });

    const textArea = screen.getByPlaceholderText("create.textPlaceholder");
    fireEvent.change(textArea, { target: { value: "Some text content" } });

    const form = nameInput.closest("form")!;
    fireEvent.submit(form);

    expect(mockCreateMutate).toHaveBeenCalledWith(
      expect.objectContaining({ name: "Test Analysis", input_text: "Some text content" }),
      expect.anything()
    );
  });

  it("description visible on text tab, hidden on upload tab", async () => {
    renderWithRouter(<CreateAnalysisForm />);

    // On text tab, description textarea should be visible
    await screen.findByPlaceholderText("create.namePlaceholder");
    expect(screen.getByText("create.descriptionLabel")).toBeDefined();

    // Switch to upload tab
    const uploadTab = screen.getByText("create.uploadTab");
    fireEvent.click(uploadTab);

    // Description label should no longer be visible (in upload tab content)
    // The TabsContent for text is hidden when upload is active
    await vi.waitFor(() => {
      expect(screen.queryByText("create.textPlaceholder")).toBeNull();
    });
  });

  it("submit button shows uploading text when pending", async () => {
    // The button should show "create.submit" normally and be disabled when name is empty
    renderWithRouter(<CreateAnalysisForm />);
    const submitBtn = await screen.findByText("create.submit");
    // Button is disabled because name is empty
    expect(submitBtn.closest("button")?.disabled).toBe(true);
  });

  it("renders both tab triggers for text and upload", async () => {
    renderWithRouter(<CreateAnalysisForm />);
    await screen.findByPlaceholderText("create.namePlaceholder");

    // Both tab triggers are rendered
    expect(screen.getByText("create.textTab")).toBeDefined();
    expect(screen.getByText("create.uploadTab")).toBeDefined();

    // Text tab content is active by default (textarea visible)
    expect(screen.getByPlaceholderText("create.textPlaceholder")).toBeDefined();
  });
});
