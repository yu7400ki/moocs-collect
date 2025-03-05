import { atom } from "jotai";
import { derive, soon } from "jotai-derive";
import type { Page } from "../schemas/page";
import { uniqueKey } from "../services/lectures";
import { getPages } from "../services/pages";
import { lectureSelectAtom } from "./lecture";

export const pagesAtom = atom((get) => {
  const lecture = get(lectureSelectAtom);
  return lecture ? getPages(lecture) : null;
});

export const pageMapAtom = derive([pagesAtom], (pages) => {
  return pages ? new Map(pages.map((page) => [page.id, page])) : null;
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
    if (lecture && (!page || pageMap?.has(page.id))) {
      set(internalPageSelectAtom, (old) => {
        const map = new Map(old);
        map.set(uniqueKey(lecture), page);
        return map;
      });
    }
  },
);

export const pageSelectIdAtom = atom(
  (get) => {
    return soon(get(pageSelectAtom), (page) => page?.id ?? null);
  },
  async (get, set, id: Page["id"] | null) => {
    const map = await get(pageMapAtom);
    const page = id ? map?.get(id) : null;
    set(pageSelectAtom, page ?? null);
  },
);
