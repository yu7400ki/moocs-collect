import { downloadSlides as downloadSlidesCommand } from "@/command/download-slides";
import type { Page } from "@/features/course/schemas/page";

export async function downloadSlides(page: Page) {
  await downloadSlidesCommand({
    year: page.lecture.course.year,
    courseId: page.lecture.course.id,
    lectureId: page.lecture.id,
    pageId: page.id,
  });
}
