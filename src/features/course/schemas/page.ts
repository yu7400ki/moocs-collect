import { z } from "zod";
import { lectureSchema } from "./lecture";

export const pageSchema = z.object({
  id: z.string().brand("PageId"),
  title: z.string(),
  lecture: lectureSchema,
});

export function castPageId(id: string): z.infer<typeof pageSchema>["id"] {
  return id as z.infer<typeof pageSchema>["id"];
}

export type Page = z.infer<typeof pageSchema>;
