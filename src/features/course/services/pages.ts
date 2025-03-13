import { getPages as getPagesCommand } from "@/command/get-pages";
import { memoizeAsync } from "@/utils/cache";
import type { Lecture } from "../schemas/lecture";
import { type Page, pageSchema } from "../schemas/page";
import { uniqueKey as lectureUniqueKey } from "./lectures";

export function uniqueKey(page: Page) {
  return `${page.lecture.course.year}-${page.lecture.course.id}-${page.lecture.id}-${page.id}`;
}

async function _getPages(lecture: Lecture) {
  const pages = await getPagesCommand({
    year: lecture.course.year,
    courseId: lecture.course.id,
    lectureId: lecture.id,
  });
  return pages.map((page) =>
    pageSchema.parse({
      ...page,
      lecture: lecture,
    }),
  );
}

export const getPages = memoizeAsync(_getPages, {
  getCacheKey: lectureUniqueKey,
});

getPages.cache;
