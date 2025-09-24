import { z } from "zod";

export const courseSchema = z.object({
  year: z.number(),
  slug: z.string().brand("CourseSlug"),
  name: z.string(),
});

export function castCourseSlug(
  slug: string,
): z.infer<typeof courseSchema>["slug"] {
  return slug as z.infer<typeof courseSchema>["slug"];
}

export type Course = z.infer<typeof courseSchema>;
