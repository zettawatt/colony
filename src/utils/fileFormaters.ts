import { FileObj } from "../classes/FileObj";

export function totalFileSizeCounter(files: FileObj[]) {
  const totalSize = files.reduce((sum, file) => sum + (file.fileSize || 0), 0);
  const kb = 1024, mb = kb * 1024, gb = mb * 1024;
  let formatted = formatFileSize(totalSize);
  return formatted
}

export function formatFileSize(size: number): string {
  if (!size) return "0 B";
  const kb = 1024, mb = kb * 1024, gb = mb * 1024;
  if (size >= gb) return (size/gb).toFixed(2) + ' GB';
  if (size >= mb) return (size/mb).toFixed(2) + ' MB';
  if (size >= kb) return (size/kb).toFixed(2) + ' KB';
  return size + ' B';
}