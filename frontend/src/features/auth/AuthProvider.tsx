import { createContext, useCallback } from "react";
import {
  useQuery,
  useMutation,
  useQueryClient,
} from "@tanstack/react-query";
import {
  fetchCurrentUser,
  loginUser,
  logoutUser,
  registerUser,
} from "./api";
import type { LoginRequest, RegisterRequest, UserProfile } from "./types";
import { AuthLoadingScreen } from "./components/AuthLoadingScreen";

export interface AuthContextType {
  user: UserProfile | null;
  isAuthenticated: boolean;
  isLoading: boolean;
  login: (data: LoginRequest) => Promise<void>;
  logout: () => Promise<void>;
  register: (data: RegisterRequest) => Promise<UserProfile>;
}

export const AuthContext = createContext<AuthContextType | null>(null);

export function AuthProvider({ children }: { children: React.ReactNode }) {
  const queryClient = useQueryClient();

  const {
    data: user,
    isLoading,
    isError,
  } = useQuery({
    queryKey: ["auth", "me"],
    queryFn: fetchCurrentUser,
    staleTime: 5 * 60 * 1000,
    retry: false,
  });

  const loginMutation = useMutation({
    mutationFn: loginUser,
    onSuccess: (response) => {
      queryClient.setQueryData(["auth", "me"], response.user);
    },
  });

  const logoutMutation = useMutation({
    mutationFn: logoutUser,
  });

  const registerMutation = useMutation({
    mutationFn: registerUser,
  });

  const login = useCallback(
    async (data: LoginRequest) => {
      await loginMutation.mutateAsync(data);
    },
    [loginMutation],
  );

  const logout = useCallback(async () => {
    queryClient.setQueryData(["auth", "me"], null);
    try {
      await logoutMutation.mutateAsync();
    } catch {
      // Cookie cleared optimistically — ignore server errors
    }
  }, [queryClient, logoutMutation]);

  const register = useCallback(
    async (data: RegisterRequest): Promise<UserProfile> => {
      return registerMutation.mutateAsync(data);
    },
    [registerMutation],
  );

  if (isLoading) {
    return <AuthLoadingScreen />;
  }

  if (isError) {
    return (
      <div className="flex h-screen flex-col items-center justify-center gap-4">
        <p className="text-muted-foreground">
          Failed to check authentication status.
        </p>
        <button
          className="text-sm underline"
          onClick={() =>
            queryClient.invalidateQueries({ queryKey: ["auth", "me"] })
          }
        >
          Retry
        </button>
      </div>
    );
  }

  const value: AuthContextType = {
    user: user ?? null,
    isAuthenticated: !!user,
    isLoading,
    login,
    logout,
    register,
  };

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}
