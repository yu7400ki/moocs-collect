import { yearAtom } from "@/features/settings/atoms/year";
import { unwrapPromise } from "@/utils/atom";
import { atom } from "jotai";
import { derive } from "jotai-derive";
import type { Course } from "../schemas/course";
import { getCourses } from "../services/courses";

const internalCoursesAtom = atom((get) => {
  const year = get(yearAtom);
  return getCourses({ year });
});

export const coursesAtom = unwrapPromise(internalCoursesAtom);

export const courseMapAtom = derive([coursesAtom], (courses) => {
  return new Map(courses.map((course) => [course.id, course]));
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
    if (!course || courseMap.has(course.id)) {
      set(internalCourseSelectAtom, (old) => {
        const map = new Map(old);
        map.set(year, course);
        return map;
      });
    }
  },
);

export const courseSelectIdAtom = atom(
  (get) => get(courseSelectAtom)?.id ?? null,
  async (get, set, id: Course["id"] | null) => {
    const map = await get(courseMapAtom);
    const course = id ? map.get(id) : null;
    set(courseSelectAtom, course ?? null);
  },
);
