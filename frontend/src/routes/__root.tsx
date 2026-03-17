import { createRootRoute, Link, Outlet } from "@tanstack/react-router";
import { TanStackRouterDevtools } from "@tanstack/router-devtools";
import { useTranslation } from "react-i18next";
import { Globe } from "lucide-react";
import { Button } from "@/components/ui/button";

export const Route = createRootRoute({
  component: RootLayout,
});

function RootLayout() {
  const { t, i18n } = useTranslation();

  const toggleLanguage = () => {
    const newLang = i18n.language === "en" ? "nb" : "en";
    i18n.changeLanguage(newLang);
  };

  return (
    <div className="min-h-screen bg-background p-4 md:p-6 lg:p-8">
      <div className="min-h-[calc(100vh-2rem)] md:min-h-[calc(100vh-3rem)] lg:min-h-[calc(100vh-4rem)] border-2 border-border rounded-lg overflow-hidden bg-background shadow-lg">
        <header className="sticky top-0 z-50 w-full border-b-2 border-border bg-background/95 backdrop-blur supports-[backdrop-filter]:bg-background/60">
          <div className="container flex h-16 items-center px-6">
            <div className="mr-4 flex">
              <Link to="/" className="mr-6 flex items-center space-x-2">
                <div className="w-6 h-6 border-2 border-primary rounded flex items-center justify-center">
                  <div className="w-2 h-2 bg-accent rounded-full" />
                </div>
                <span className="font-bold font-mono tracking-tight">{t("appName")}</span>
              </Link>
              <nav className="flex items-center space-x-6 text-sm font-medium">
                <Link
                  to="/"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  {t("nav.home")}
                </Link>
                <Link
                  to="/frameworks"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  Frameworks
                </Link>
                <Link
                  to="/crosswalk"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  Crosswalk
                </Link>
                <Link
                  to="/ontology"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  {t("nav.ontology")}
                </Link>
                <Link
                  to="/compliance"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  {t("nav.compliance")}
                </Link>
                <Link
                  to="/reports"
                  className="transition-colors hover:text-foreground/80 text-foreground/60 [&.active]:text-foreground font-mono"
                >
                  {t("nav.reports")}
                </Link>
              </nav>
            </div>
            <div className="flex flex-1 items-center justify-end space-x-2">
              <Button variant="ghost" size="icon" onClick={toggleLanguage}>
                <Globe className="h-4 w-4" />
                <span className="sr-only">Toggle language</span>
              </Button>
            </div>
          </div>
        </header>
        <main className="container py-6 px-6">
          <Outlet />
        </main>
        <TanStackRouterDevtools />
      </div>
    </div>
  );
}
