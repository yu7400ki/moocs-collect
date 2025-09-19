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
