import { unwrapPromise } from "@/utils/atom";
import { atom } from "jotai";
import { derive } from "jotai-derive";
import type { Course } from "../schemas/course";
import { getCourses } from "../services/courses";
import { yearAtom } from "./year";

const internalCoursesAtom = atom((get) => {
  const year = get(yearAtom);
  return getCourses({ year });
});

export const coursesAtom = unwrapPromise(internalCoursesAtom);

export const courseMapAtom = derive([coursesAtom], (courses) => {
  return new Map(courses.map((course) => [course.slug, course]));
});

const internalCourseSelectAtom = atom<Map<number | undefined, Course | null>>(
  new Map(),
);

export const courseSelectAtom = atom(
  (get) => {
    const year = get(yearAtom);
    const map = get(internalCourseSelectAtom);
    return map.get(year) ?? null;
  },
  async (get, set, course: Course | null) => {
    const courseMap = await get(courseMapAtom);
    const year = get(yearAtom);
    if (!course || courseMap.has(course.slug)) {
      set(internalCourseSelectAtom, (old) => {
        const map = new Map(old);
        map.set(year, course);
        return map;
      });
    }
  },
);

export const courseSelectSlugAtom = atom(
  (get) => get(courseSelectAtom)?.slug ?? null,
  async (get, set, slug: Course["slug"] | null) => {
    const map = await get(courseMapAtom);
    const course = slug ? map.get(slug) : null;
    set(courseSelectAtom, course ?? null);
  },
);
