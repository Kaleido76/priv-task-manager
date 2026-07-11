import { DEFAULT_LOCALE } from "$config";

export function formatDateTime(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleString(DEFAULT_LOCALE, {
    year: "numeric", month: "2-digit", day: "2-digit",
    hour: "2-digit", minute: "2-digit",
  });
}

export function formatDate(iso: string): string {
  const d = new Date(iso);
  return d.toLocaleString(DEFAULT_LOCALE, {
    year: "numeric", month: "2-digit", day: "2-digit",
  });
}

export function nowISO(): string {
  return new Date().toISOString();
}

export function truncatePath(path: string, maxLen: number = 60): string {
  if (path.length <= maxLen) return path;
  const sep = path.includes("\\") ? "\\" : "/";
  const idx = path.lastIndexOf(sep);
  if (idx < 0) return path;
  const fileName = path.slice(idx + 1);
  const drive = path.slice(0, path.indexOf(sep) + 1);
  const ellipsis = "…";
  const head = drive;
  if (head.length + ellipsis.length + fileName.length > maxLen) {
    if (fileName.length + 3 > maxLen) return fileName.slice(0, maxLen - 3) + "...";
    return ellipsis + fileName;
  }
  return head + ellipsis + fileName;
}
