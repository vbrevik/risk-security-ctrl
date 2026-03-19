import { describe, it, expect, vi } from "vitest";
import { renderHook } from "@testing-library/react";
import React from "react";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";

vi.mock("../api", () => ({
  fetchCurrentUser: vi.fn(),
  loginUser: vi.fn(),
  logoutUser: vi.fn(),
  registerUser: vi.fn(),
}));

import { fetchCurrentUser } from "../api";
const mockedFetchCurrentUser = vi.mocked(fetchCurrentUser);

import { useHasPermission } from "../useHasPermission";
import { AuthProvider } from "../AuthProvider";

function createWrapper(mockUser: { id: string; email: string; name: string; role: string } | null) {
  mockedFetchCurrentUser.mockResolvedValue(mockUser);
  const queryClient = new QueryClient({
    defaultOptions: { queries: { retry: false } },
  });
  return function Wrapper({ children }: { children: React.ReactNode }) {
    return React.createElement(
      QueryClientProvider,
      { client: queryClient },
      React.createElement(AuthProvider, null, children),
    );
  };
}

describe("useHasPermission", () => {
  it("returns true for admin on any feature/action", async () => {
    const wrapper = createWrapper({ id: "1", email: "a@b.c", name: "A", role: "admin" });
    const { result } = renderHook(() => useHasPermission("compliance", "delete"), { wrapper });
    await vi.waitFor(() => {
      expect(result.current).toBe(true);
    });
  });

  it("returns false for viewer on write actions", async () => {
    const wrapper = createWrapper({ id: "1", email: "a@b.c", name: "V", role: "viewer" });
    const { result: r1 } = renderHook(() => useHasPermission("compliance", "create"), { wrapper });
    await vi.waitFor(() => {
      expect(r1.current).toBe(false);
    });
    const { result: r2 } = renderHook(() => useHasPermission("compliance", "delete"), { wrapper });
    await vi.waitFor(() => {
      expect(r2.current).toBe(false);
    });
  });

  it("returns true for specialist on compliance:create", async () => {
    const wrapper = createWrapper({ id: "1", email: "a@b.c", name: "S", role: "specialist" });
    const { result } = renderHook(() => useHasPermission("compliance", "create"), { wrapper });
    await vi.waitFor(() => {
      expect(result.current).toBe(true);
    });
  });

  it("returns true for specialist on compliance:read", async () => {
    const wrapper = createWrapper({ id: "1", email: "a@b.c", name: "S", role: "specialist" });
    const { result } = renderHook(() => useHasPermission("compliance", "read"), { wrapper });
    await vi.waitFor(() => {
      expect(result.current).toBe(true);
    });
  });

  it("returns false when not authenticated", async () => {
    const wrapper = createWrapper(null);
    const { result } = renderHook(() => useHasPermission("compliance", "read"), { wrapper });
    await vi.waitFor(() => {
      expect(result.current).toBe(false);
    });
  });
});
