import {
  getRecordedCourses as getRecordedCoursesCommand,
  type RecordedCourse,
} from "@/command/get-recorded-courses";
import {
  type HighlightedText,
  type SearchSlidesArgs,
  type SlideSearchEntry,
  searchSlides as searchSlidesCommand,
} from "@/command/search-slides";

export const searchSlides = async (
  args: SearchSlidesArgs,
): Promise<SlideSearchEntry[]> => {
  return await searchSlidesCommand(args);
};

export type { SlideSearchEntry, HighlightedText };

export const getRecordedCourses = async (): Promise<RecordedCourse[]> => {
  return await getRecordedCoursesCommand();
};
