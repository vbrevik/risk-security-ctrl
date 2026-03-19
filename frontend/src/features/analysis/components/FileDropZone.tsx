import { useRef, useState } from "react";
import { useTranslation } from "react-i18next";
import { Upload, X } from "lucide-react";
import { Button } from "@/components/ui/button";

interface FileDropZoneProps {
  accept: string[];
  maxSizeMB: number;
  selectedFile: File | null;
  onFileSelected: (file: File) => void;
  onClear: () => void;
  onError: (msg: string) => void;
}

export function FileDropZone({
  accept,
  maxSizeMB,
  selectedFile,
  onFileSelected,
  onClear,
  onError,
}: FileDropZoneProps) {
  const { t } = useTranslation("analysis");
  const [isDragging, setIsDragging] = useState(false);
  const dragCounter = useRef(0);
  const inputRef = useRef<HTMLInputElement>(null);

  function validateFile(file: File): boolean {
    const ext = "." + file.name.split(".").pop()?.toLowerCase();
    const validExts = accept.map((a) => a.toLowerCase());
    const validMimes: Record<string, string> = {
      ".pdf": "application/pdf",
      ".docx": "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
    };
    const isValidExt = validExts.includes(ext);
    const isValidMime = Object.values(validMimes).includes(file.type);
    if (!isValidExt || !isValidMime) {
      onError(t("create.invalidFileType"));
      return false;
    }
    if (file.size > maxSizeMB * 1024 * 1024) {
      onError(t("create.fileTooLarge"));
      return false;
    }
    return true;
  }

  function handleFile(file: File) {
    if (validateFile(file)) {
      onFileSelected(file);
    }
  }

  function handleDragEnter(e: React.DragEvent) {
    e.preventDefault();
    dragCounter.current++;
    setIsDragging(true);
  }

  function handleDragLeave(e: React.DragEvent) {
    e.preventDefault();
    dragCounter.current--;
    if (dragCounter.current <= 0) {
      dragCounter.current = 0;
      setIsDragging(false);
    }
  }

  function handleDragOver(e: React.DragEvent) {
    e.preventDefault();
  }

  function handleDrop(e: React.DragEvent) {
    e.preventDefault();
    dragCounter.current = 0;
    setIsDragging(false);
    const file = e.dataTransfer.files[0];
    if (file) handleFile(file);
  }

  function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    if (file) handleFile(file);
    if (inputRef.current) inputRef.current.value = "";
  }

  if (selectedFile) {
    const sizeMB = (selectedFile.size / (1024 * 1024)).toFixed(1);
    return (
      <div className="rounded-lg border-2 border-dashed border-border p-6 flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Upload className="h-5 w-5 text-muted-foreground" />
          <div>
            <p className="text-sm font-medium">{selectedFile.name}</p>
            <p className="text-xs text-muted-foreground">{sizeMB} MB</p>
          </div>
        </div>
        <Button variant="ghost" size="icon" onClick={onClear}>
          <X className="h-4 w-4" />
        </Button>
      </div>
    );
  }

  return (
    <div
      className={`rounded-lg border-2 border-dashed p-8 text-center transition-colors ${
        isDragging
          ? "border-primary bg-primary/5"
          : "border-border hover:border-muted-foreground/50"
      }`}
      onDragEnter={handleDragEnter}
      onDragLeave={handleDragLeave}
      onDragOver={handleDragOver}
      onDrop={handleDrop}
    >
      <Upload className="h-8 w-8 text-muted-foreground/50 mx-auto mb-3" />
      <p className="text-sm text-muted-foreground">
        {t("create.dropzoneText")}{" "}
        <label
          htmlFor="file-upload"
          className="text-primary cursor-pointer hover:underline"
        >
          {t("create.dropzoneBrowse")}
        </label>
      </p>
      <p className="text-xs text-muted-foreground mt-1">{t("create.maxFileSize")}</p>
      <input
        ref={inputRef}
        id="file-upload"
        type="file"
        accept={accept.join(",")}
        className="sr-only"
        onChange={handleInputChange}
      />
    </div>
  );
}
