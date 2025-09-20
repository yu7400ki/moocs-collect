import type { LatestRelease } from "../types/release";

export async function getLatestRelease(): Promise<LatestRelease> {
  if (import.meta.env.DEV) {
    const response = await fetch(
      "https://yu7400ki.github.io/moocs-collect/latest.json",
    );
    if (!response.ok) {
      throw new Error("Failed to fetch latest release");
    }
    return response.json();
  }
  const glob = import.meta.glob("../../public/latest.json", { eager: true });
  const module = glob["../../public/latest.json"] as
    | { default: LatestRelease }
    | undefined;
  if (!module) {
    throw new Error("Failed to load latest release");
  }
  return module.default;
}
