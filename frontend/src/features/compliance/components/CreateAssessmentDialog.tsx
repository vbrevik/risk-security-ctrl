import { useState } from "react";
import { useTranslation } from "react-i18next";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { useFrameworks } from "@/features/ontology/api";
import { useCreateAssessment } from "../api";

interface CreateAssessmentDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function CreateAssessmentDialog({
  open,
  onOpenChange,
}: CreateAssessmentDialogProps) {
  const { t } = useTranslation("compliance");
  const { data: frameworks } = useFrameworks();
  const createMutation = useCreateAssessment();

  const [name, setName] = useState("");
  const [frameworkId, setFrameworkId] = useState("");
  const [description, setDescription] = useState("");
  const [dueDate, setDueDate] = useState("");

  const resetForm = () => {
    setName("");
    setFrameworkId("");
    setDescription("");
    setDueDate("");
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!name.trim() || !frameworkId) return;

    createMutation.mutate(
      {
        framework_id: frameworkId,
        name: name.trim(),
        description: description.trim() || undefined,
        due_date: dueDate || undefined,
      },
      {
        onSuccess: () => {
          resetForm();
          onOpenChange(false);
        },
      }
    );
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("create.title")}</DialogTitle>
          <DialogDescription>{t("create.description")}</DialogDescription>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="assessment-name">{t("create.nameLabel")}</Label>
            <Input
              id="assessment-name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder={t("create.namePlaceholder")}
              required
            />
          </div>

          <div className="space-y-2">
            <Label>{t("create.frameworkLabel")}</Label>
            <Select value={frameworkId} onValueChange={setFrameworkId} required>
              <SelectTrigger className="w-full">
                <SelectValue
                  placeholder={t("create.frameworkPlaceholder")}
                />
              </SelectTrigger>
              <SelectContent>
                {frameworks?.map((fw) => (
                  <SelectItem key={fw.id} value={fw.id}>
                    {fw.name}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>

          <div className="space-y-2">
            <Label htmlFor="assessment-description">
              {t("create.descriptionLabel")}
            </Label>
            <Input
              id="assessment-description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder={t("create.descriptionPlaceholder")}
            />
          </div>

          <div className="space-y-2">
            <Label htmlFor="assessment-due-date">
              {t("create.dueDateLabel")}
            </Label>
            <Input
              id="assessment-due-date"
              type="date"
              value={dueDate}
              onChange={(e) => setDueDate(e.target.value)}
            />
          </div>

          <DialogFooter>
            <Button
              type="button"
              variant="outline"
              onClick={() => onOpenChange(false)}
            >
              {t("create.cancel")}
            </Button>
            <Button
              type="submit"
              disabled={
                !name.trim() || !frameworkId || createMutation.isPending
              }
            >
              {createMutation.isPending
                ? t("create.creating")
                : t("create.submit")}
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
