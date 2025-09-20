interface Platform {
  signature: string;
  url: string;
}

export interface LatestRelease {
  version: string;
  notes: string;
  pub_date: string;
  platforms: Record<string, Platform>;
}
