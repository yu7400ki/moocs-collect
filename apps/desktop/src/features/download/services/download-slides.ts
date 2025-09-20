import { downloadSlides as downloadSlidesCommand } from "@/command/download-slides";
import type { Page } from "@/features/course/schemas/page";

export async function downloadSlides(page: Page) {
  return await downloadSlidesCommand({
    year: page.year,
    courseSlug: page.courseSlug,
    lectureSlug: page.lectureSlug,
    pageSlug: page.slug,
  });
}
