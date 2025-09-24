import { createCommand } from "./utils";

export type SearchSlidesArgs = {
  query: string;
  filters: string[];
};

export type HighlightedText = {
  text: string;
  isHighlighted: boolean;
};

type SearchResult = {
  pageKey: string;
  facet: string;
  contentSnippet: string;
  highlightedContent: HighlightedText[];
  score: number;
};

export type SlideSearchEntry = {
  searchResult: SearchResult;
  year: number;
  courseName: string;
  lectureName: string;
  pageName: string;
  downloadPath?: string;
};

export const searchSlides = createCommand<SearchSlidesArgs, SlideSearchEntry[]>(
  "search_slides",
);
