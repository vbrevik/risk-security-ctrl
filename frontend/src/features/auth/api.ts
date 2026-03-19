import { api } from "@/lib/api";
import type {
  AuthResponse,
  LoginRequest,
  RegisterRequest,
  UserProfile,
} from "./types";

export async function fetchCurrentUser(): Promise<UserProfile | null> {
  try {
    const { data } = await api.get<UserProfile>("/auth/me");
    return data;
  } catch (error: unknown) {
    if (
      error &&
      typeof error === "object" &&
      "response" in error &&
      (error as { response?: { status?: number } }).response?.status === 401
    ) {
      return null;
    }
    throw error;
  }
}

export async function loginUser(data: LoginRequest): Promise<AuthResponse> {
  const response = await api.post<AuthResponse>("/auth/login", data);
  return response.data;
}

export async function registerUser(
  data: RegisterRequest,
): Promise<UserProfile> {
  const response = await api.post<UserProfile>("/auth/register", data);
  return response.data;
}

export async function logoutUser(): Promise<void> {
  await api.post("/auth/logout");
}
