import { Link } from "@tanstack/react-router";
import { useTranslation } from "react-i18next";
import { LogOut } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";
import { useAuth } from "@/features/auth/useAuth";

export function NavbarAuthControls() {
  const { user, isAuthenticated, logout } = useAuth();
  const { t } = useTranslation("auth");

  if (!isAuthenticated || !user) {
    return (
      <Link
        to="/login"
        className="text-xs font-medium text-foreground/50 hover:text-foreground/80 transition-colors"
      >
        {t("login.title", { defaultValue: "Sign In" })}
      </Link>
    );
  }

  return (
    <div className="flex items-center gap-2">
      <span className="text-xs font-medium font-mono text-foreground/70">
        {user.name}
      </span>
      <Badge variant="outline" className="text-[10px] px-1.5 py-0">
        {user.role}
      </Badge>
      <Button
        variant="ghost"
        size="icon"
        className="h-7 w-7"
        onClick={() => logout()}
        title={t("navbar.logout", { defaultValue: "Sign Out" })}
      >
        <LogOut className="h-3.5 w-3.5" />
        <span className="sr-only">
          {t("navbar.logout", { defaultValue: "Sign Out" })}
        </span>
      </Button>
    </div>
  );
}
