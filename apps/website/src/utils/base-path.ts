// Trailing slash stripped: "/moocs-collect/" → "/moocs-collect", "/" → ""
export const base = import.meta.env.BASE_URL.replace(/\/$/, "");

// Resolves an absolute path relative to the Vite base.
// basePath("/guide") → "/moocs-collect/guide"
// basePath("/")      → "/moocs-collect/"
export function basePath(path: string): string {
  return base + path;
}
