import { atomWithDebounce } from "@/utils/atom";
import { type SetStateAction, atom } from "jotai";
import { derive } from "jotai-derive";
import { loadable, unwrap } from "jotai/utils";
import { searchSlides } from "../services/search";

const searchQueryDebounced = atomWithDebounce("", 300);

export const searchQueryAtom = atom(
  (get) => get(searchQueryDebounced.currentValueAtom),
  (_, set, value: SetStateAction<string>) => {
    set(searchQueryDebounced.debouncedValueAtom, value);
  },
);

const searchParamsAtom = atom((get) => {
  const query = get(searchQueryDebounced.debouncedValueAtom);

  return {
    query: query.trim().replace(/\s+/g, " "),
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
