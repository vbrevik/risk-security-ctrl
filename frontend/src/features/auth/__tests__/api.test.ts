import { describe, it, expect, vi, beforeEach } from "vitest";

vi.mock("@/lib/api", () => ({
  api: {
    get: vi.fn(),
    post: vi.fn(),
    put: vi.fn(),
    delete: vi.fn(),
    defaults: {
      withCredentials: true,
      headers: {
        common: { "X-Requested-With": "XMLHttpRequest" },
        "Content-Type": "application/json",
      },
    },
  },
}));

import { api } from "@/lib/api";
const mockedApi = vi.mocked(api);

import {
  fetchCurrentUser,
  loginUser,
  registerUser,
  logoutUser,
} from "../api";

describe("Auth API functions", () => {
  beforeEach(() => {
    vi.clearAllMocks();
  });

  describe("fetchCurrentUser", () => {
    it("returns UserProfile on 200", async () => {
      const user = {
        id: "1",
        email: "test@example.com",
        name: "Test User",
        role: "admin",
      };
      mockedApi.get.mockResolvedValueOnce({ data: user });

      const result = await fetchCurrentUser();

      expect(mockedApi.get).toHaveBeenCalledWith("/auth/me");
      expect(result).toEqual(user);
    });

    it("returns null on 401", async () => {
      mockedApi.get.mockRejectedValueOnce({
        response: { status: 401 },
      });

      const result = await fetchCurrentUser();

      expect(result).toBeNull();
    });

    it("re-throws non-401 errors", async () => {
      mockedApi.get.mockRejectedValueOnce({
        response: { status: 500 },
      });

      await expect(fetchCurrentUser()).rejects.toEqual({
        response: { status: 500 },
      });
    });
  });

  describe("loginUser", () => {
    it("sends POST with credentials and returns AuthResponse", async () => {
      const credentials = {
        email: "test@example.com",
        password: "password123",
      };
      const authResponse = {
        token: "abc123",
        user: {
          id: "1",
          email: "test@example.com",
          name: "Test User",
          role: "viewer",
        },
      };
      mockedApi.post.mockResolvedValueOnce({ data: authResponse });

      const result = await loginUser(credentials);

      expect(mockedApi.post).toHaveBeenCalledWith("/auth/login", credentials);
      expect(result).toEqual(authResponse);
    });
  });

  describe("registerUser", () => {
    it("sends POST and returns UserProfile", async () => {
      const registerData = {
        email: "new@example.com",
        name: "New User",
        password: "password123",
      };
      const userProfile = {
        id: "2",
        email: "new@example.com",
        name: "New User",
        role: "viewer",
      };
      mockedApi.post.mockResolvedValueOnce({ data: userProfile });

      const result = await registerUser(registerData);

      expect(mockedApi.post).toHaveBeenCalledWith(
        "/auth/register",
        registerData,
      );
      expect(result).toEqual(userProfile);
    });
  });

  describe("logoutUser", () => {
    it("sends POST to /auth/logout", async () => {
      mockedApi.post.mockResolvedValueOnce({});

      await logoutUser();

      expect(mockedApi.post).toHaveBeenCalledWith("/auth/logout");
    });
  });
});

describe("Axios instance configuration", () => {
  it("has withCredentials: true", () => {
    expect(api.defaults.withCredentials).toBe(true);
  });

  it("has X-Requested-With: XMLHttpRequest header", () => {
    expect(api.defaults.headers.common["X-Requested-With"]).toBe(
      "XMLHttpRequest",
    );
  });
});
