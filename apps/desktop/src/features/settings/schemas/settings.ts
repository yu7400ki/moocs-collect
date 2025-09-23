import { z } from "zod";

export const settingsSchema = z.object({
  version: z.literal(1).default(1),
  theme: z.enum(["system", "light", "dark"]),
  downloadDir: z.string(),
});

export type Settings = z.infer<typeof settingsSchema>;
