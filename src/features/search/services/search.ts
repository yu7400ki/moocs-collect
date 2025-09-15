import {
  type HighlightedText,
  type SearchResult,
  type SearchSlidesArgs,
  searchSlides as searchSlidesCommand,
} from "@/command/search-slides";

export const searchSlides = async (
  args: SearchSlidesArgs,
): Promise<SearchResult[]> => {
  return await searchSlidesCommand(args);
};

export type { SearchResult, HighlightedText };
