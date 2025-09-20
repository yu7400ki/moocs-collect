import { getLectures as getLecturesCommand } from "@/command/get-lectures";
import { memoizeAsync } from "@/utils/cache";
import type { Course } from "../schemas/course";
import {
  type Lecture,
  type LectureGroup,
  lectureGroupSchema,
} from "../schemas/lecture";
import { uniqueKey as courseUniqueKey } from "./courses";

export function uniqueKey(lecture: Lecture) {
  return `${lecture.year}-${lecture.courseSlug}-${lecture.slug}`;
}

export function lectureGroupUniqueKey(group: LectureGroup) {
  return `${group.year}-${group.courseSlug}-${group.name}`;
}

async function _getLectureGroups(course: Course) {
  const lectureGroups = await getLecturesCommand({
    year: course.year,
    courseSlug: course.slug,
  });
  return lectureGroups.map((group) => lectureGroupSchema.parse(group));
}

export const getLectureGroups = memoizeAsync(_getLectureGroups, {
  getCacheKey: courseUniqueKey,
});

export async function getAllLectures(course: Course): Promise<Lecture[]> {
  const groups = await getLectureGroups(course);
  return groups.flatMap((group) => group.lectures);
}
