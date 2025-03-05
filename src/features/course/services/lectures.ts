import { getLectures as getLecturesCommand } from "@/command/get-lectures";
import { memoizeAsync } from "@/utils/cache";
import type { Course } from "../schemas/course";
import { type Lecture, lectureSchema } from "../schemas/lecture";
import { uniqueKey as courseUniqueKey } from "./courses";

export function uniqueKey(lecture: Lecture) {
  return `${lecture.course.year}-${lecture.course.id}-${lecture.id}`;
}

async function _getLectures(course: Course) {
  const lectures = await getLecturesCommand({
    year: course.year,
    courseId: course.id,
  });
  return lectures.map((lecture) =>
    lectureSchema.parse({
      ...lecture,
      course: course,
    }),
  );
}

export const getLectures = memoizeAsync(_getLectures, {
  getCacheKey: courseUniqueKey,
});
