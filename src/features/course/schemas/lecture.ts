import { z } from "zod";
import { courseSchema } from "./course";

export const lectureSchema = z.object({
  id: z.string().brand("LectureId"),
  name: z.string(),
  group: z.string(),
  course: courseSchema,
});

export function castLectureId(id: string): z.infer<typeof lectureSchema>["id"] {
  return id as z.infer<typeof lectureSchema>["id"];
}

export type Lecture = z.infer<typeof lectureSchema>;
