export const listenFileDrop = (
  target: HTMLElement,
  onFile: (file: File) => void
) => {
  const DRAGGING = "dragging";
  const hasFiles = (e: DragEvent) =>
    !!e.dataTransfer && Array.from(e.dataTransfer.types).includes("Files");

  target.addEventListener("dragover", (e) => {
    if (!hasFiles(e)) return;
    e.preventDefault();
    e.dataTransfer!.dropEffect = "copy";
    target.classList.add(DRAGGING);
  });
  target.addEventListener("dragleave", (e) => {
    if (e.relatedTarget && target.contains(e.relatedTarget as Node)) return;
    target.classList.remove(DRAGGING);
  });
  target.addEventListener("drop", (e) => {
    if (!hasFiles(e)) return;
    e.preventDefault();
    target.classList.remove(DRAGGING);
    const file = e.dataTransfer!.files[0];
    if (file) onFile(file);
  });
};

export const download = (filename: string, content: string) => {
  const url = URL.createObjectURL(new Blob([content], { type: "text/plain" }));
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.click();
  // revoking synchronously may cancel the download in some browsers
  setTimeout(() => URL.revokeObjectURL(url));
};
