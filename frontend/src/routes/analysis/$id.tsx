import { createFileRoute, Link } from "@tanstack/react-router";

export const Route = createFileRoute("/analysis/$id")({
  component: AnalysisDetailPage,
});

function AnalysisDetailPage() {
  const { id } = Route.useParams();
  return (
    <div>
      <Link to="/analysis">&larr; Back</Link>
      <p>Analysis detail page for {id} — coming in split 02</p>
    </div>
  );
}
