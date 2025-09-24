import { atom, type SetStateAction } from "jotai";
import { loadable, unwrap } from "jotai/utils";
import { derive } from "jotai-derive";
import { atomWithDebounce } from "@/utils/atom";
import { getRecordedCourses, searchSlides } from "../services/search";

const searchQueryDebounced = atomWithDebounce("", 300);

export const searchQueryAtom = atom(
  (get) => get(searchQueryDebounced.currentValueAtom),
  (_, set, value: SetStateAction<string>) => {
    set(searchQueryDebounced.debouncedValueAtom, value);
  },
);

export const facetFilterAtom = atom<string[]>([]);

const searchParamsAtom = atom((get) => {
  const query = get(searchQueryDebounced.debouncedValueAtom);
  const filters = get(facetFilterAtom);

  return {
    query: query.trim().replace(/\s+/g, " "),
    filters,
  };
});

const internalSearchResultsAtom = atom(async (get) => {
  const params = get(searchParamsAtom);

  try {
    const value = await searchSlides(params);
    return {
      ok: true as const,
      value,
    };
  } catch (error) {
    return {
      ok: false as const,
      reason:
        error instanceof Error
          ? error.message
          : typeof error === "string"
            ? error
            : "Unknown error",
    };
  }
});

export const searchResultsAtom = unwrap(
  derive([internalSearchResultsAtom], (r) => (r.ok ? r.value : [])),
  (prev) => prev ?? [],
);

export const searchErrorAtom = unwrap(
  derive([internalSearchResultsAtom], (r) => (r.ok ? null : r.reason)),
  (prev) => prev ?? null,
);

export const isSearchingAtom = atom((get) => {
  const loadableAtom = loadable(internalSearchResultsAtom);
  const loadableState = get(loadableAtom);
  return loadableState.state === "loading";
});

const refreshTriggerAtom = atom({});
const internalRecordedCoursesAtom = atom(
  (get) => {
    get(refreshTriggerAtom);
    return getRecordedCourses();
  },
  (_, set) => {
    set(refreshTriggerAtom, {});
  },
);

export const recordedCoursesAtom = unwrap(
  internalRecordedCoursesAtom,
  (prev) => prev ?? [],
);

export const groupedRecordedCoursesAtom = atom(
  (get) => {
    const courses = get(recordedCoursesAtom);
    const grouped = new Map<number, typeof courses>();
    for (const course of courses) {
      if (!grouped.has(course.year)) {
        grouped.set(course.year, []);
      }
      grouped.get(course.year)?.push(course);
    }
    return grouped;
  },
  (_, set) => set(recordedCoursesAtom),
);
