import { z } from "zod";

export const settingsSchema = z.object({
  theme: z.enum(["system", "light", "dark"]),
  downloadDir: z.string(),
});

export type Settings = z.infer<typeof settingsSchema>;
