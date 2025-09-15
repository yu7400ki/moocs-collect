import { createCommand } from "./utils";

export type SearchSlidesArgs = {
  query: string;
  yearFilter?: number;
  coursesFilter?: string[];
};

export type HighlightedText = {
  text: string;
  isHighlighted: boolean;
};

export type SearchResult = {
  pageKey: string;
  year: number;
  course: string;
  lecture: string;
  page: string;
  contentSnippet: string;
  highlightedContent: HighlightedText[];
  score: number;
};

export const searchSlides = createCommand<SearchSlidesArgs, SearchResult[]>(
  "search_slides",
);
