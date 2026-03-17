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
          <div className="container flex h-12 items-center px-6 overflow-x-auto">
            <Link to="/" className="mr-6 flex items-center space-x-2 flex-shrink-0">
              <div className="w-5 h-5 border-2 border-primary rounded flex items-center justify-center">
                <div className="w-1.5 h-1.5 bg-accent rounded-full" />
              </div>
              <span className="font-bold font-mono text-sm tracking-tight">{t("appName")}</span>
            </Link>
            <nav className="flex items-center text-xs font-medium font-mono whitespace-nowrap">
              <Link
                to="/"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.home")}
              </Link>
              <Link
                to="/ontology"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.ontology")}
              </Link>
              <span className="text-border mx-1.5">·</span>
              <Link
                to="/frameworks"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.frameworks")}
              </Link>
              <Link
                to="/crosswalk"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.crosswalk")}
              </Link>
              <Link
                to="/landscape"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.landscape")}
              </Link>
              <Link
                to="/concepts/search"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.search")}
              </Link>
              <span className="text-border mx-1.5">·</span>
              <Link
                to="/compliance"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.compliance")}
              </Link>
              <Link
                to="/reports"
                className="transition-colors hover:text-foreground/80 text-foreground/50 [&.active]:text-foreground px-2.5 py-1"
              >
                {t("nav.reports")}
              </Link>
            </nav>
            <div className="flex flex-1 items-center justify-end space-x-2 flex-shrink-0">
              <Button variant="ghost" size="icon" onClick={toggleLanguage} className="h-7 w-7">
                <Globe className="h-3.5 w-3.5" />
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
