import { getCourses as getCourseCommand } from "@/command/get-courses";
import { memoizeAsync } from "@/utils/cache";
import { type Course, courseSchema } from "../schemas/course";

export function uniqueKey(course: Course) {
  return `${course.year}-${course.id}`;
}

async function _getCourses() {
  const courses = await getCourseCommand();
  return courses.map((course) => courseSchema.parse(course));
}

export const getCourses = memoizeAsync(_getCourses);
