import type { QueryClient } from "@tanstack/react-query";
import type { AuthContextType } from "./AuthProvider";

export interface RouterContext {
  auth: AuthContextType;
  queryClient: QueryClient;
}
