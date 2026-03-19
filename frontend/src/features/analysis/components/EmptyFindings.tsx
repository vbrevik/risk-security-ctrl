import { useTranslation } from "react-i18next";
import { Link } from "@tanstack/react-router";
import { SearchX } from "lucide-react";
import { Button } from "@/components/ui/button";

export function EmptyFindings() {
  const { t } = useTranslation("analysis");

  return (
    <div className="flex flex-col items-center justify-center py-16 text-center">
      <SearchX className="h-12 w-12 text-muted-foreground" />
      <h3 className="text-lg font-semibold mt-4">
        {t("findings.empty.title")}
      </h3>
      <p className="text-muted-foreground mt-2 max-w-md">
        {t("findings.empty.description")}
      </p>
      <Button variant="outline" className="mt-4" asChild>
        <Link to="/analysis/settings">
          {t("findings.empty.settingsLink")}
        </Link>
      </Button>
    </div>
  );
}
