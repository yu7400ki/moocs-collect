import { unwrapPromise } from "@/utils/atom";
import { atom } from "jotai";
import { derive } from "jotai-derive";
import type { Lecture } from "../schemas/lecture";
import { uniqueKey } from "../services/courses";
import { getAllLectures, getLectureGroups } from "../services/lectures";
import { courseSelectAtom } from "./course";

export const internalLectureGroupsAtom = atom((get) => {
  const course = get(courseSelectAtom);
  return course ? getLectureGroups(course) : null;
});

export const lectureGroupsAtom = unwrapPromise(internalLectureGroupsAtom);

// 後方互換性のために、全てのlectureを平坦化したatom
export const internalLecturesAtom = atom((get) => {
  const course = get(courseSelectAtom);
  return course ? getAllLectures(course) : null;
});

export const lecturesAtom = unwrapPromise(internalLecturesAtom);

export const lectureMapAtom = derive([lecturesAtom], (lectures) => {
  return lectures
    ? new Map(lectures.map((lecture) => [lecture.slug, lecture]))
    : null;
});

const internalLectureSelectAtom = atom<Map<string, Lecture | null>>(new Map());

export const lectureSelectAtom = atom(
  (get) => {
    const course = get(courseSelectAtom);
    const map = get(internalLectureSelectAtom);
    return course ? (map.get(uniqueKey(course)) ?? null) : null;
  },
  async (get, set, lecture: Lecture | null) => {
    const lectureMap = await get(lectureMapAtom);
    const course = get(courseSelectAtom);
    if (course && (!lecture || lectureMap?.has(lecture.slug))) {
      set(internalLectureSelectAtom, (old) => {
        const map = new Map(old);
        map.set(uniqueKey(course), lecture);
        return map;
      });
    }
  },
);

export const lectureSelectSlugAtom = atom(
  (get) => get(lectureSelectAtom)?.slug ?? null,
  async (get, set, slug: Lecture["slug"] | null) => {
    const map = await get(lectureMapAtom);
    const lecture = slug ? map?.get(slug) : null;
    set(lectureSelectAtom, lecture ?? null);
  },
);
