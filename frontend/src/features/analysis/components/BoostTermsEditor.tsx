import { useTranslation } from "react-i18next";
import { Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

interface BoostTerm {
  term: string;
  weight: number;
}

interface BoostTermsEditorProps {
  value: BoostTerm[];
  onChange: (terms: BoostTerm[]) => void;
}

export function BoostTermsEditor({ value, onChange }: BoostTermsEditorProps) {
  const { t } = useTranslation("analysis");

  function handleTermChange(index: number, term: string) {
    const updated = [...value];
    updated[index] = { ...updated[index], term };
    onChange(updated);
  }

  function handleWeightChange(index: number, weight: number) {
    const updated = [...value];
    updated[index] = { ...updated[index], weight };
    onChange(updated);
  }

  function handleAdd() {
    onChange([...value, { term: "", weight: 1.0 }]);
  }

  function handleDelete(index: number) {
    onChange(value.filter((_, i) => i !== index));
  }

  return (
    <div className="space-y-2">
      {value.map((entry, index) => (
        <div key={index} className="flex items-center gap-2">
          <Input
            placeholder={t("settings.termLabel")}
            value={entry.term}
            onChange={(e) => handleTermChange(index, e.target.value)}
            aria-invalid={!entry.term.trim()}
            className="flex-1"
          />
          <Input
            type="number"
            placeholder={t("settings.weightLabel")}
            value={entry.weight}
            onChange={(e) => handleWeightChange(index, parseFloat(e.target.value) || 0)}
            step={0.1}
            min={0}
            className="w-24"
          />
          <Button
            variant="ghost"
            size="icon"
            onClick={() => handleDelete(index)}
          >
            <Trash2 className="h-4 w-4" />
          </Button>
        </div>
      ))}
      <Button variant="outline" size="sm" onClick={handleAdd}>
        {t("settings.addTerm")}
      </Button>
    </div>
  );
}
