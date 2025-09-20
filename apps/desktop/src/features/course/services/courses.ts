import { getCourses as getCourseCommand } from "@/command/get-courses";
import { memoizeAsync } from "@/utils/cache";
import { type Course, courseSchema } from "../schemas/course";

export function uniqueKey(course: Course) {
  return `${course.year}-${course.slug}`;
}

async function _getCourses(args: { year?: number } = {}) {
  const courses = await getCourseCommand(args);
  return courses.map((course) => courseSchema.parse(course));
}

export const getCourses = memoizeAsync(_getCourses, {
  getCacheKey: (args) => args?.year,
});
