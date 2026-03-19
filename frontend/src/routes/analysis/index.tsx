import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/")({
  component: AnalysisListPage,
});

function AnalysisListPage() {
  return <div>Analysis list page — implemented in section-04</div>;
}
