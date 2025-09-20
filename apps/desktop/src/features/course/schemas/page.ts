import { z } from "zod";

export const pageSchema = z.object({
  year: z.number(),
  courseSlug: z.string().brand("CourseSlug"),
  lectureSlug: z.string().brand("LectureSlug"),
  slug: z.string().brand("PageSlug"),
  name: z.string(),
});

export function castPageSlug(slug: string): z.infer<typeof pageSchema>["slug"] {
  return slug as z.infer<typeof pageSchema>["slug"];
}

export type Page = z.infer<typeof pageSchema>;
