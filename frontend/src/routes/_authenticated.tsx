import { createFileRoute, Outlet } from "@tanstack/react-router";
import { requireAuth } from "@/features/auth/authGuard";

export const Route = createFileRoute("/_authenticated")({
  beforeLoad: ({ context }) => {
    requireAuth(context);
  },
  component: () => <Outlet />,
});
