import { atom } from "jotai";
import { derive } from "jotai-derive";
import type { Course } from "../schemas/course";
import { getCourses } from "../services/courses";

export const coursesAtom = atom(async () => {
  return await getCourses();
});

export const courseMapAtom = derive([coursesAtom], (courses) => {
  return new Map(courses.map((course) => [course.id, course]));
});

const internalCourseSelectAtom = atom<Course | null>(null);

export const courseSelectAtom = atom(
  (get) => get(internalCourseSelectAtom),
  async (get, set, course: Course | null) => {
    if (course) {
      const map = await get(courseMapAtom);
      map.has(course.id) && set(internalCourseSelectAtom, course);
    } else {
      set(internalCourseSelectAtom, null);
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
