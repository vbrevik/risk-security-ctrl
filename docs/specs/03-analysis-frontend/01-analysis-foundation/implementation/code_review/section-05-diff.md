diff --git a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
index a779162..32557db 100644
--- a/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
+++ b/docs/specs/03-analysis-frontend/01-analysis-foundation/implementation/deep_implement_config.json
@@ -26,6 +26,10 @@
     "section-03-shadcn-components": {
       "status": "complete",
       "commit_hash": "46ab818"
+    },
+    "section-04-list-page": {
+      "status": "complete",
+      "commit_hash": "f8d67f1"
     }
   },
   "pre_commit": {
diff --git a/frontend/src/features/analysis/components/CreateAnalysisForm.tsx b/frontend/src/features/analysis/components/CreateAnalysisForm.tsx
new file mode 100644
index 0000000..397039a
--- /dev/null
+++ b/frontend/src/features/analysis/components/CreateAnalysisForm.tsx
@@ -0,0 +1,140 @@
+import { useState } from "react";
+import { useTranslation } from "react-i18next";
+import { useNavigate } from "@tanstack/react-router";
+import { Loader2 } from "lucide-react";
+import { Button } from "@/components/ui/button";
+import { Input } from "@/components/ui/input";
+import { Label } from "@/components/ui/label";
+import { Textarea } from "@/components/ui/textarea";
+import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
+import { useCreateAnalysis, useUploadAnalysis } from "../api";
+import { FileDropZone } from "./FileDropZone";
+
+export function CreateAnalysisForm() {
+  const { t } = useTranslation("analysis");
+  const navigate = useNavigate();
+
+  const createMutation = useCreateAnalysis();
+  const { progress, ...uploadMutation } = useUploadAnalysis();
+
+  const [activeTab, setActiveTab] = useState<"text" | "upload">("text");
+  const [name, setName] = useState("");
+  const [description, setDescription] = useState("");
+  const [inputText, setInputText] = useState("");
+  const [selectedFile, setSelectedFile] = useState<File | null>(null);
+  const [fileError, setFileError] = useState<string | null>(null);
+
+  const isPending = createMutation.isPending || uploadMutation.isPending;
+  const isDisabled = isPending || !name.trim();
+
+  function handleSubmit(e: React.FormEvent) {
+    e.preventDefault();
+
+    if (!name.trim()) return;
+
+    if (activeTab === "text") {
+      if (!inputText.trim()) return;
+      createMutation.mutate(
+        {
+          name: name.trim(),
+          description: description.trim() || undefined,
+          input_text: inputText,
+        },
+        {
+          onSuccess: (data) => {
+            navigate({ to: "/analysis/$id", params: { id: data.id } });
+          },
+        }
+      );
+    } else {
+      if (!selectedFile) return;
+      uploadMutation.mutate(
+        { file: selectedFile, name: name.trim() },
+        {
+          onSuccess: (data) => {
+            navigate({ to: "/analysis/$id", params: { id: data.id } });
+          },
+        }
+      );
+    }
+  }
+
+  return (
+    <form onSubmit={handleSubmit} className="space-y-6">
+      <div className="space-y-2">
+        <Label htmlFor="name">{t("create.nameLabel")}</Label>
+        <Input
+          id="name"
+          placeholder={t("create.namePlaceholder")}
+          value={name}
+          onChange={(e) => setName(e.target.value)}
+        />
+      </div>
+
+      <Tabs value={activeTab} onValueChange={(v) => setActiveTab(v as "text" | "upload")}>
+        <TabsList>
+          <TabsTrigger value="text">{t("create.textTab")}</TabsTrigger>
+          <TabsTrigger value="upload">{t("create.uploadTab")}</TabsTrigger>
+        </TabsList>
+
+        <TabsContent value="text" className="space-y-4 mt-4">
+          <div className="space-y-2">
+            <Label htmlFor="description">{t("create.descriptionLabel")}</Label>
+            <Input
+              id="description"
+              value={description}
+              onChange={(e) => setDescription(e.target.value)}
+            />
+          </div>
+          <div className="space-y-2">
+            <Textarea
+              placeholder={t("create.textPlaceholder")}
+              value={inputText}
+              onChange={(e) => setInputText(e.target.value)}
+              rows={10}
+            />
+          </div>
+        </TabsContent>
+
+        <TabsContent value="upload" className="space-y-4 mt-4">
+          <FileDropZone
+            accept={[".pdf", ".docx"]}
+            maxSizeMB={25}
+            selectedFile={selectedFile}
+            onFileSelected={setSelectedFile}
+            onClear={() => {
+              setSelectedFile(null);
+              setFileError(null);
+            }}
+            onError={setFileError}
+          />
+          {fileError && (
+            <p className="text-sm text-destructive">{fileError}</p>
+          )}
+          {uploadMutation.isPending && (
+            <div className="space-y-1">
+              <div className="h-2 rounded-full bg-muted overflow-hidden">
+                <div
+                  className="h-full rounded-full bg-primary transition-all"
+                  style={{ width: `${progress}%` }}
+                />
+              </div>
+              <p className="text-xs text-muted-foreground text-right">{progress}%</p>
+            </div>
+          )}
+        </TabsContent>
+      </Tabs>
+
+      <Button type="submit" disabled={isDisabled} className="w-full">
+        {isPending ? (
+          <>
+            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
+            {t("create.uploading")}
+          </>
+        ) : (
+          t("create.submit")
+        )}
+      </Button>
+    </form>
+  );
+}
diff --git a/frontend/src/features/analysis/components/FileDropZone.tsx b/frontend/src/features/analysis/components/FileDropZone.tsx
new file mode 100644
index 0000000..15a6eba
--- /dev/null
+++ b/frontend/src/features/analysis/components/FileDropZone.tsx
@@ -0,0 +1,132 @@
+import { useRef, useState } from "react";
+import { useTranslation } from "react-i18next";
+import { Upload, X } from "lucide-react";
+import { Button } from "@/components/ui/button";
+
+interface FileDropZoneProps {
+  accept: string[];
+  maxSizeMB: number;
+  selectedFile: File | null;
+  onFileSelected: (file: File) => void;
+  onClear: () => void;
+  onError: (msg: string) => void;
+}
+
+export function FileDropZone({
+  accept,
+  maxSizeMB,
+  selectedFile,
+  onFileSelected,
+  onClear,
+  onError,
+}: FileDropZoneProps) {
+  const { t } = useTranslation("analysis");
+  const [isDragging, setIsDragging] = useState(false);
+  const dragCounter = useRef(0);
+  const inputRef = useRef<HTMLInputElement>(null);
+
+  function validateFile(file: File): boolean {
+    const ext = "." + file.name.split(".").pop()?.toLowerCase();
+    const validExts = accept.map((a) => a.toLowerCase());
+    if (!validExts.includes(ext)) {
+      onError(t("create.invalidFileType"));
+      return false;
+    }
+    if (file.size > maxSizeMB * 1024 * 1024) {
+      onError(t("create.fileTooLarge"));
+      return false;
+    }
+    return true;
+  }
+
+  function handleFile(file: File) {
+    if (validateFile(file)) {
+      onFileSelected(file);
+    }
+  }
+
+  function handleDragEnter(e: React.DragEvent) {
+    e.preventDefault();
+    dragCounter.current++;
+    setIsDragging(true);
+  }
+
+  function handleDragLeave(e: React.DragEvent) {
+    e.preventDefault();
+    dragCounter.current--;
+    if (dragCounter.current <= 0) {
+      dragCounter.current = 0;
+      setIsDragging(false);
+    }
+  }
+
+  function handleDragOver(e: React.DragEvent) {
+    e.preventDefault();
+  }
+
+  function handleDrop(e: React.DragEvent) {
+    e.preventDefault();
+    dragCounter.current = 0;
+    setIsDragging(false);
+    const file = e.dataTransfer.files[0];
+    if (file) handleFile(file);
+  }
+
+  function handleInputChange(e: React.ChangeEvent<HTMLInputElement>) {
+    const file = e.target.files?.[0];
+    if (file) handleFile(file);
+    if (inputRef.current) inputRef.current.value = "";
+  }
+
+  if (selectedFile) {
+    const sizeMB = (selectedFile.size / (1024 * 1024)).toFixed(1);
+    return (
+      <div className="rounded-lg border-2 border-dashed border-border p-6 flex items-center justify-between">
+        <div className="flex items-center gap-3">
+          <Upload className="h-5 w-5 text-muted-foreground" />
+          <div>
+            <p className="text-sm font-medium">{selectedFile.name}</p>
+            <p className="text-xs text-muted-foreground">{sizeMB} MB</p>
+          </div>
+        </div>
+        <Button variant="ghost" size="icon" onClick={onClear}>
+          <X className="h-4 w-4" />
+        </Button>
+      </div>
+    );
+  }
+
+  return (
+    <div
+      className={`rounded-lg border-2 border-dashed p-8 text-center transition-colors ${
+        isDragging
+          ? "border-primary bg-primary/5"
+          : "border-border hover:border-muted-foreground/50"
+      }`}
+      onDragEnter={handleDragEnter}
+      onDragLeave={handleDragLeave}
+      onDragOver={handleDragOver}
+      onDrop={handleDrop}
+    >
+      <Upload className="h-8 w-8 text-muted-foreground/50 mx-auto mb-3" />
+      <p className="text-sm text-muted-foreground">
+        {t("create.dropzoneText")}{" "}
+        <label
+          htmlFor="file-upload"
+          className="text-primary cursor-pointer hover:underline"
+        >
+          {t("create.dropzoneBrowse")}
+        </label>
+      </p>
+      <p className="text-xs text-muted-foreground mt-1">{t("create.maxFileSize")}</p>
+      <input
+        ref={inputRef}
+        id="file-upload"
+        type="file"
+        accept={accept.join(",")}
+        className="sr-only"
+        onChange={handleInputChange}
+      />
+    </div>
+  );
+}
diff --git a/frontend/src/features/analysis/components/__tests__/CreateAnalysisForm.test.tsx b/frontend/src/features/analysis/components/__tests__/CreateAnalysisForm.test.tsx
new file mode 100644
index 0000000..4f35c30
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/CreateAnalysisForm.test.tsx
@@ -0,0 +1,104 @@
+import { describe, it, expect, vi, beforeEach } from "vitest";
+import { render, screen, fireEvent } from "@testing-library/react";
+import React from "react";
+import {
+  createRootRoute,
+  createRoute,
+  createRouter,
+  createMemoryHistory,
+  RouterProvider,
+  Outlet,
+} from "@tanstack/react-router";
+import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
+import { CreateAnalysisForm } from "../CreateAnalysisForm";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+const mockCreateMutate = vi.fn();
+const mockUploadMutate = vi.fn();
+
+vi.mock("@/features/analysis/api", () => ({
+  useCreateAnalysis: () => ({
+    mutate: mockCreateMutate,
+    isPending: false,
+  }),
+  useUploadAnalysis: () => ({
+    mutate: mockUploadMutate,
+    isPending: false,
+    progress: 0,
+  }),
+}));
+
+function renderWithRouter(ui: React.ReactElement) {
+  const rootRoute = createRootRoute({
+    component: () => React.createElement("div", null, React.createElement(Outlet)),
+  });
+
+  const indexRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/",
+    component: () => ui,
+  });
+
+  const detailRoute = createRoute({
+    getParentRoute: () => rootRoute,
+    path: "/analysis/$id",
+    component: () => React.createElement("div", null, "Detail"),
+  });
+
+  const router = createRouter({
+    routeTree: rootRoute.addChildren([indexRoute, detailRoute]),
+    history: createMemoryHistory({ initialEntries: ["/"] }),
+  });
+
+  const queryClient = new QueryClient({ defaultOptions: { queries: { retry: false } } });
+
+  return render(
+    React.createElement(
+      QueryClientProvider,
+      { client: queryClient },
+      React.createElement(RouterProvider, { router: router as any })
+    )
+  );
+}
+
+describe("CreateAnalysisForm", () => {
+  beforeEach(() => {
+    vi.clearAllMocks();
+  });
+
+  it("renders name input and tab toggle", async () => {
+    renderWithRouter(<CreateAnalysisForm />);
+    expect(await screen.findByPlaceholderText("create.namePlaceholder")).toBeDefined();
+    expect(screen.getByText("create.textTab")).toBeDefined();
+    expect(screen.getByText("create.uploadTab")).toBeDefined();
+  });
+
+  it("submit disabled when name is empty", async () => {
+    renderWithRouter(<CreateAnalysisForm />);
+    const submitBtn = await screen.findByText("create.submit");
+    expect(submitBtn.closest("button")?.disabled).toBe(true);
+  });
+
+  it("calls createAnalysis mutation on text tab submit", async () => {
+    renderWithRouter(<CreateAnalysisForm />);
+
+    const nameInput = await screen.findByPlaceholderText("create.namePlaceholder");
+    fireEvent.change(nameInput, { target: { value: "Test Analysis" } });
+
+    const textArea = screen.getByPlaceholderText("create.textPlaceholder");
+    fireEvent.change(textArea, { target: { value: "Some text content" } });
+
+    const form = nameInput.closest("form")!;
+    fireEvent.submit(form);
+
+    expect(mockCreateMutate).toHaveBeenCalledWith(
+      expect.objectContaining({ name: "Test Analysis", input_text: "Some text content" }),
+      expect.anything()
+    );
+  });
+});
diff --git a/frontend/src/features/analysis/components/__tests__/FileDropZone.test.tsx b/frontend/src/features/analysis/components/__tests__/FileDropZone.test.tsx
new file mode 100644
index 0000000..3dd8d37
--- /dev/null
+++ b/frontend/src/features/analysis/components/__tests__/FileDropZone.test.tsx
@@ -0,0 +1,89 @@
+import { describe, it, expect, vi } from "vitest";
+import { render, screen, fireEvent } from "@testing-library/react";
+import { FileDropZone } from "../FileDropZone";
+
+vi.mock("react-i18next", () => ({
+  useTranslation: () => ({
+    t: (key: string) => key,
+  }),
+}));
+
+const defaultProps = {
+  accept: [".pdf", ".docx"],
+  maxSizeMB: 25,
+  selectedFile: null,
+  onFileSelected: vi.fn(),
+  onClear: vi.fn(),
+  onError: vi.fn(),
+};
+
+describe("FileDropZone", () => {
+  it("renders drop zone with browse link", () => {
+    render(<FileDropZone {...defaultProps} />);
+    expect(screen.getByText("create.dropzoneText")).toBeDefined();
+    expect(screen.getByText("create.dropzoneBrowse")).toBeDefined();
+  });
+
+  it("accepts PDF files via input change", () => {
+    const onFileSelected = vi.fn();
+    render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
+    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
+    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
+    fireEvent.change(input, { target: { files: [file] } });
+    expect(onFileSelected).toHaveBeenCalledWith(file);
+  });
+
+  it("accepts DOCX files via input change", () => {
+    const onFileSelected = vi.fn();
+    render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
+    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
+    const file = new File(["content"], "test.docx", {
+      type: "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
+    });
+    fireEvent.change(input, { target: { files: [file] } });
+    expect(onFileSelected).toHaveBeenCalledWith(file);
+  });
+
+  it("rejects files exceeding maxSizeMB", () => {
+    const onError = vi.fn();
+    const onFileSelected = vi.fn();
+    render(<FileDropZone {...defaultProps} onError={onError} onFileSelected={onFileSelected} />);
+    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
+    const bigFile = new File(["x".repeat(26 * 1024 * 1024)], "big.pdf", { type: "application/pdf" });
+    Object.defineProperty(bigFile, "size", { value: 26 * 1024 * 1024 });
+    fireEvent.change(input, { target: { files: [bigFile] } });
+    expect(onError).toHaveBeenCalled();
+    expect(onFileSelected).not.toHaveBeenCalled();
+  });
+
+  it("rejects non-PDF/DOCX files", () => {
+    const onError = vi.fn();
+    const onFileSelected = vi.fn();
+    render(<FileDropZone {...defaultProps} onError={onError} onFileSelected={onFileSelected} />);
+    const input = document.querySelector('input[type="file"]') as HTMLInputElement;
+    const txtFile = new File(["content"], "test.txt", { type: "text/plain" });
+    fireEvent.change(input, { target: { files: [txtFile] } });
+    expect(onError).toHaveBeenCalled();
+    expect(onFileSelected).not.toHaveBeenCalled();
+  });
+
+  it("shows drag highlight on dragEnter, removes on dragLeave", () => {
+    const { container } = render(<FileDropZone {...defaultProps} />);
+    const dropZone = container.firstElementChild as HTMLElement;
+    fireEvent.dragEnter(dropZone, { dataTransfer: { items: [] } });
+    expect(dropZone.className).toMatch(/border-primary/);
+    fireEvent.dragLeave(dropZone);
+    expect(dropZone.className).not.toMatch(/border-primary/);
+  });
+
+  it("calls onFileSelected with dropped file", () => {
+    const onFileSelected = vi.fn();
+    const { container } = render(<FileDropZone {...defaultProps} onFileSelected={onFileSelected} />);
+    const dropZone = container.firstElementChild as HTMLElement;
+    const file = new File(["content"], "test.pdf", { type: "application/pdf" });
+    fireEvent.drop(dropZone, {
+      dataTransfer: { files: [file] },
+    });
+    expect(onFileSelected).toHaveBeenCalledWith(file);
+  });
+});
diff --git a/frontend/src/features/analysis/index.ts b/frontend/src/features/analysis/index.ts
index 891d295..1603236 100644
--- a/frontend/src/features/analysis/index.ts
+++ b/frontend/src/features/analysis/index.ts
@@ -3,3 +3,5 @@ export * from "./api";
 export { StatusBadge } from "./components/StatusBadge";
 export { AnalysisCard } from "./components/AnalysisCard";
 export { AnalysisList } from "./components/AnalysisList";
+export { CreateAnalysisForm } from "./components/CreateAnalysisForm";
+export { FileDropZone } from "./components/FileDropZone";
diff --git a/frontend/src/routes/analysis/create.tsx b/frontend/src/routes/analysis/create.tsx
index 6853f02..cd1906b 100644
--- a/frontend/src/routes/analysis/create.tsx
+++ b/frontend/src/routes/analysis/create.tsx
@@ -1,9 +1,24 @@
-import { createFileRoute } from "@tanstack/react-router";
+import { createFileRoute, Link } from "@tanstack/react-router";
+import { useTranslation } from "react-i18next";
+import { CreateAnalysisForm } from "@/features/analysis/components/CreateAnalysisForm";
 
 export const Route = createFileRoute("/analysis/create")({
   component: CreateAnalysisPage,
 });
 
 function CreateAnalysisPage() {
-  return <div>Create analysis page — implemented in section-05</div>;
+  const { t } = useTranslation("analysis");
+
+  return (
+    <div className="max-w-2xl mx-auto p-6 space-y-6">
+      <Link
+        to="/analysis"
+        className="text-sm text-muted-foreground hover:text-foreground transition-colors"
+      >
+        &larr; {t("common.back")}
+      </Link>
+      <h1 className="text-2xl font-bold tracking-tight">{t("create.title")}</h1>
+      <CreateAnalysisForm />
+    </div>
+  );
 }
