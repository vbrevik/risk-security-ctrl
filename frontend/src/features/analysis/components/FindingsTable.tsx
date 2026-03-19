import React from "react";
import { useTranslation } from "react-i18next";
import { ChevronRight } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import type { AnalysisFinding, FindingType, FindingsFilter } from "../types";
import { FindingTypeTag } from "./FindingTypeTag";

interface FindingsTableProps {
  findings: AnalysisFinding[];
  expandedIds: Set<string>;
  onToggleExpand: (id: string) => void;
  frameworkIds: string[];
  filters: FindingsFilter;
  onFilterChange: (filters: FindingsFilter) => void;
  page: number;
  totalPages: number;
  onPageChange: (page: number) => void;
  onConceptClick?: (conceptId: string) => void;
}

const ALL = "__all__";

const FINDING_TYPES: FindingType[] = [
  "addressed",
  "partially_addressed",
  "gap",
  "not_applicable",
];

export function FindingsTable({
  findings,
  expandedIds,
  onToggleExpand,
  frameworkIds,
  filters,
  onFilterChange,
  page,
  totalPages,
  onPageChange,
  onConceptClick,
}: FindingsTableProps) {
  const { t } = useTranslation("analysis");

  return (
    <div className="space-y-4">
      {/* Filters */}
      <div className="flex flex-wrap gap-4">
        <Select
          value={filters.framework_id ?? ALL}
          onValueChange={(v) =>
            onFilterChange({
              ...filters,
              framework_id: v === ALL ? undefined : v,
            })
          }
        >
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder={t("findings.filters.allFrameworks")} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL}>
              {t("findings.filters.allFrameworks")}
            </SelectItem>
            {frameworkIds.map((fw) => (
              <SelectItem key={fw} value={fw}>
                {fw}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={filters.finding_type ?? ALL}
          onValueChange={(v) =>
            onFilterChange({
              ...filters,
              finding_type: v === ALL ? undefined : (v as FindingType),
            })
          }
        >
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder={t("findings.filters.allTypes")} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL}>
              {t("findings.filters.allTypes")}
            </SelectItem>
            {FINDING_TYPES.map((ft) => (
              <SelectItem key={ft} value={ft}>
                {t(`findings.type.${ft}`)}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>

        <Select
          value={filters.priority != null ? String(filters.priority) : ALL}
          onValueChange={(v) =>
            onFilterChange({
              ...filters,
              priority: v === ALL ? undefined : Number(v),
            })
          }
        >
          <SelectTrigger className="w-[180px]">
            <SelectValue placeholder={t("findings.filters.allPriorities")} />
          </SelectTrigger>
          <SelectContent>
            <SelectItem value={ALL}>
              {t("findings.filters.allPriorities")}
            </SelectItem>
            {[1, 2, 3, 4].map((p) => (
              <SelectItem key={p} value={String(p)}>
                P{p}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      </div>

      {/* Table */}
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-10" />
            <TableHead>{t("findings.columns.conceptCode")}</TableHead>
            <TableHead>{t("findings.columns.conceptName")}</TableHead>
            <TableHead>{t("findings.columns.framework")}</TableHead>
            <TableHead>{t("findings.columns.type")}</TableHead>
            <TableHead>{t("findings.columns.priority")}</TableHead>
            <TableHead>{t("findings.columns.confidence")}</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {findings.map((finding) => (
            <React.Fragment key={finding.id}>
              <TableRow>
                <TableCell>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={() => onToggleExpand(finding.id)}
                    aria-expanded={expandedIds.has(finding.id)}
                    aria-controls={`finding-detail-${finding.id}`}
                    aria-label={
                      expandedIds.has(finding.id)
                        ? t("findings.collapse")
                        : t("findings.expand")
                    }
                  >
                    <ChevronRight
                      className={`h-4 w-4 transition-transform ${
                        expandedIds.has(finding.id) ? "rotate-90" : ""
                      }`}
                    />
                  </Button>
                </TableCell>
                <TableCell className="font-mono text-sm">
                  {onConceptClick && finding.concept_code ? (
                    <button
                      className="text-left hover:underline text-accent-foreground cursor-pointer"
                      onClick={() => onConceptClick(finding.concept_id)}
                    >
                      {finding.concept_code}
                    </button>
                  ) : (
                    finding.concept_code ?? "\u2014"
                  )}
                </TableCell>
                <TableCell>
                  {onConceptClick && finding.concept_name ? (
                    <button
                      className="text-left hover:underline text-accent-foreground cursor-pointer"
                      onClick={() => onConceptClick(finding.concept_id)}
                    >
                      {finding.concept_name}
                    </button>
                  ) : (
                    finding.concept_name ?? "\u2014"
                  )}
                </TableCell>
                <TableCell>{finding.framework_id}</TableCell>
                <TableCell>
                  <FindingTypeTag type={finding.finding_type} />
                </TableCell>
                <TableCell>P{finding.priority}</TableCell>
                <TableCell>
                  {Math.round(finding.confidence_score * 100)}%
                </TableCell>
              </TableRow>
              {expandedIds.has(finding.id) && (
                <TableRow
                  id={`finding-detail-${finding.id}`}
                  className="bg-muted"
                >
                  <TableCell colSpan={7} className="p-4">
                    <div className="space-y-3 text-sm">
                      <div>
                        <p className="font-semibold">
                          {t("findings.evidence")}
                        </p>
                        <p className="mt-1">
                          {finding.evidence_text ?? "\u2014"}
                        </p>
                      </div>
                      <div>
                        <p className="font-semibold">
                          {t("findings.recommendation")}
                        </p>
                        <p className="mt-1">
                          {finding.recommendation ?? "\u2014"}
                        </p>
                      </div>
                      <div>
                        <p className="font-semibold">
                          {t("findings.conceptDefinition")}
                        </p>
                        <p className="mt-1">
                          {finding.concept_definition ?? "\u2014"}
                        </p>
                      </div>
                      {finding.concept_code && (
                        <div>
                          <p className="font-semibold">
                            {t("findings.sourceReference")}
                          </p>
                          <p className="mt-1 font-mono">
                            {finding.concept_code}
                          </p>
                        </div>
                      )}
                    </div>
                  </TableCell>
                </TableRow>
              )}
            </React.Fragment>
          ))}
        </TableBody>
      </Table>

      {/* Pagination */}
      <div className="flex items-center justify-between">
        <p className="text-sm text-muted-foreground">
          {t("list.pagination.pageOf", { page, total: totalPages })}
        </p>
        <div className="flex gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(page - 1)}
            disabled={page <= 1}
          >
            {t("list.pagination.previous")}
          </Button>
          <Button
            variant="outline"
            size="sm"
            onClick={() => onPageChange(page + 1)}
            disabled={page >= totalPages}
          >
            {t("list.pagination.next")}
          </Button>
        </div>
      </div>
    </div>
  );
}
