import { z } from "zod";

export const courseSchema = z.object({
  year: z.number(),
  id: z.string().brand("CourseId"),
  name: z.string(),
});

export function castCourseId(id: string): z.infer<typeof courseSchema>["id"] {
  return id as z.infer<typeof courseSchema>["id"];
}

export type Course = z.infer<typeof courseSchema>;
