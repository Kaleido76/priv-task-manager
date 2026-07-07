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
