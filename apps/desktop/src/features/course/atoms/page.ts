import { atom } from "jotai";
import { derive } from "jotai-derive";
import { unwrapPromise } from "@/utils/atom";
import type { Page } from "../schemas/page";
import { uniqueKey } from "../services/lectures";
import { getPages } from "../services/pages";
import { lectureSelectAtom } from "./lecture";

const internalPagesAtom = atom((get) => {
  const lecture = get(lectureSelectAtom);
  return lecture ? getPages(lecture) : null;
});

export const pagesAtom = unwrapPromise(internalPagesAtom);

export const pageMapAtom = derive([pagesAtom], (pages) => {
  return pages ? new Map(pages.map((page) => [page.slug, page])) : null;
});

const internalPageSelectAtom = atom<Map<string, Page | null>>(new Map());

export const pageSelectAtom = atom(
  (get) => {
    const lecture = get(lectureSelectAtom);
    const map = get(internalPageSelectAtom);
    return lecture ? (map.get(uniqueKey(lecture)) ?? null) : null;
  },
  async (get, set, page: Page | null) => {
    const pageMap = await get(pageMapAtom);
    const lecture = get(lectureSelectAtom);
    if (lecture && (!page || pageMap?.has(page.slug))) {
      set(internalPageSelectAtom, (old) => {
        const map = new Map(old);
        map.set(uniqueKey(lecture), page);
        return map;
      });
    }
  },
);

export const pageSelectSlugAtom = atom(
  (get) => get(pageSelectAtom)?.slug ?? null,
  async (get, set, slug: Page["slug"] | null) => {
    const map = await get(pageMapAtom);
    const page = slug ? map?.get(slug) : null;
    set(pageSelectAtom, page ?? null);
  },
);
