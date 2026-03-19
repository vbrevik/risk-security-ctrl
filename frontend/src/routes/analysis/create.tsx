import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/create")({
  component: CreateAnalysisPage,
});

function CreateAnalysisPage() {
  return <div>Create analysis page — implemented in section-05</div>;
}
