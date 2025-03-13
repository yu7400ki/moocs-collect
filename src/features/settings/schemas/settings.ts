import { z } from "zod";

export const settingsSchema = z.object({
  theme: z.enum(["system", "light", "dark"]),
  downloadDir: z.string(),
  year: z.coerce
    .number()
    .int()
    .max(new Date().getFullYear())
    .transform((v) => v || undefined)
    .optional(),
});

export type Settings = z.infer<typeof settingsSchema>;
