import { z } from "zod";

export const lectureSchema = z.object({
  year: z.number(),
  courseSlug: z.string().brand("CourseSlug"),
  slug: z.string().brand("LectureSlug"),
  name: z.string(),
  index: z.number(),
});

export const lectureGroupSchema = z.object({
  year: z.number(),
  courseSlug: z.string().brand("CourseSlug"),
  name: z.string(),
  lectures: z.array(lectureSchema),
  index: z.number(),
});

export function castLectureSlug(
  slug: string,
): z.infer<typeof lectureSchema>["slug"] {
  return slug as z.infer<typeof lectureSchema>["slug"];
}

export type Lecture = z.infer<typeof lectureSchema>;
export type LectureGroup = z.infer<typeof lectureGroupSchema>;
