import { getPages as getPagesCommand } from "@/command/get-pages";
import { memoizeAsync } from "@/utils/cache";
import type { Lecture } from "../schemas/lecture";
import { type Page, pageSchema } from "../schemas/page";
import { uniqueKey as lectureUniqueKey } from "./lectures";

export function uniqueKey(page: Page) {
  return `${page.year}-${page.courseSlug}-${page.lectureSlug}-${page.slug}`;
}

async function _getPages(lecture: Lecture) {
  const pages = await getPagesCommand({
    year: lecture.year,
    courseSlug: lecture.courseSlug,
    lectureSlug: lecture.slug,
  });
  return pages.map((page) => pageSchema.parse(page));
}

export const getPages = memoizeAsync(_getPages, {
  getCacheKey: lectureUniqueKey,
});

getPages.cache;
