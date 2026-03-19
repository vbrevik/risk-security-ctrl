import { Link } from "@tanstack/react-router";
import {
  Card,
  CardHeader,
  CardTitle,
  CardContent,
  CardFooter,
} from "@/components/ui/card";
import { StatusBadge } from "./StatusBadge";
import type { AnalysisListItem } from "../types";

interface AnalysisCardProps {
  analysis: AnalysisListItem;
}

export function AnalysisCard({ analysis }: AnalysisCardProps) {
  const createdDate = new Date(analysis.created_at).toLocaleDateString();

  return (
    <Link
      to="/analysis/$id"
      params={{ id: analysis.id }}
      className="block"
    >
      <Card className="transition-colors hover:border-primary/50">
        <CardHeader className="pb-2">
          <div className="flex items-start justify-between gap-2">
            <CardTitle className="text-base leading-tight">
              {analysis.name}
            </CardTitle>
            <StatusBadge status={analysis.status} />
          </div>
        </CardHeader>
        <CardContent className="pb-2">
          {analysis.description && (
            <p className="text-sm text-muted-foreground line-clamp-2">
              {analysis.description}
            </p>
          )}
          <div className="flex items-center gap-2 mt-2">
            <span className="text-xs px-1.5 py-0.5 rounded bg-muted font-mono">
              {analysis.input_type}
            </span>
            {analysis.processing_time_ms != null && (
              <span className="text-xs text-muted-foreground">
                {(analysis.processing_time_ms / 1000).toFixed(1)}s
              </span>
            )}
          </div>
        </CardContent>
        <CardFooter className="text-xs text-muted-foreground pt-0">
          <span>{createdDate}</span>
          {analysis.error_message && (
            <span className="ml-auto text-destructive truncate max-w-[200px]">
              {analysis.error_message}
            </span>
          )}
        </CardFooter>
      </Card>
    </Link>
  );
}
