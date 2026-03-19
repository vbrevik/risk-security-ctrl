import { useState } from "react";
import { useTranslation } from "react-i18next";
import { useNavigate } from "@tanstack/react-router";
import { Loader2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { useCreateAnalysis, useUploadAnalysis } from "../api";
import { FileDropZone } from "./FileDropZone";

export function CreateAnalysisForm() {
  const { t } = useTranslation("analysis");
  const navigate = useNavigate();

  const createMutation = useCreateAnalysis();
  const { progress, ...uploadMutation } = useUploadAnalysis();

  const [activeTab, setActiveTab] = useState<"text" | "upload">("text");
  const [name, setName] = useState("");
  const [description, setDescription] = useState("");
  const [inputText, setInputText] = useState("");
  const [selectedFile, setSelectedFile] = useState<File | null>(null);
  const [fileError, setFileError] = useState<string | null>(null);
  const [nameError, setNameError] = useState(false);

  const isPending = createMutation.isPending || uploadMutation.isPending;
  const isDisabled = isPending || !name.trim();

  function handleSubmit(e: React.FormEvent) {
    e.preventDefault();

    if (!name.trim()) {
      setNameError(true);
      return;
    }
    setNameError(false);

    if (activeTab === "text") {
      if (!inputText.trim()) return;
      createMutation.mutate(
        {
          name: name.trim(),
          description: description.trim() || undefined,
          input_text: inputText,
        },
        {
          onSuccess: (data) => {
            navigate({ to: "/analysis/$id", params: { id: data.id } });
          },
        }
      );
    } else {
      if (!selectedFile) return;
      uploadMutation.mutate(
        { file: selectedFile, name: name.trim() },
        {
          onSuccess: (data) => {
            navigate({ to: "/analysis/$id", params: { id: data.id } });
          },
        }
      );
    }
  }

  return (
    <form onSubmit={handleSubmit} className="space-y-6">
      <div className="space-y-2">
        <Label htmlFor="name">{t("create.nameLabel")}</Label>
        <Input
          id="name"
          placeholder={t("create.namePlaceholder")}
          value={name}
          onChange={(e) => {
            setName(e.target.value);
            if (nameError) setNameError(false);
          }}
        />
        {nameError && (
          <p className="text-sm text-destructive">{t("create.nameLabel")}</p>
        )}
      </div>

      <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as "text" | "upload")}>
        <TabsList>
          <TabsTrigger value="text">{t("create.textTab")}</TabsTrigger>
          <TabsTrigger value="upload">{t("create.uploadTab")}</TabsTrigger>
        </TabsList>

        <TabsContent value="text" className="space-y-4 mt-4">
          <div className="space-y-2">
            <Label htmlFor="description">{t("create.descriptionLabel")}</Label>
            <Textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              rows={3}
            />
          </div>
          <div className="space-y-2">
            <Textarea
              placeholder={t("create.textPlaceholder")}
              value={inputText}
              onChange={(e) => setInputText(e.target.value)}
              rows={10}
            />
          </div>
        </TabsContent>

        <TabsContent value="upload" className="space-y-4 mt-4">
          <FileDropZone
            accept={[".pdf", ".docx"]}
            maxSizeMB={25}
            selectedFile={selectedFile}
            onFileSelected={(file) => {
              setSelectedFile(file);
              setFileError(null);
            }}
            onClear={() => {
              setSelectedFile(null);
              setFileError(null);
            }}
            onError={setFileError}
          />
          {fileError && (
            <p className="text-sm text-destructive">{fileError}</p>
          )}
          {uploadMutation.isPending && (
            <div className="space-y-1">
              <div className="h-2 rounded-full bg-muted overflow-hidden">
                <div
                  className="h-full rounded-full bg-primary transition-all"
                  style={{ width: `${progress}%` }}
                />
              </div>
              <p className="text-xs text-muted-foreground text-right">{progress}%</p>
            </div>
          )}
        </TabsContent>
      </Tabs>

      <Button type="submit" disabled={isDisabled} className="w-full">
        {isPending ? (
          <>
            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
            {t("create.uploading")}
          </>
        ) : (
          t("create.submit")
        )}
      </Button>
    </form>
  );
}
