import { unwrapPromise } from "@/utils/atom";
import { atom } from "jotai";
import { derive } from "jotai-derive";
import type { Lecture } from "../schemas/lecture";
import { uniqueKey } from "../services/courses";
import { getLectures } from "../services/lectures";
import { courseSelectAtom } from "./course";

export const internalLecturesAtom = atom((get) => {
  const course = get(courseSelectAtom);
  return course ? getLectures(course) : null;
});

export const lecturesAtom = unwrapPromise(internalLecturesAtom);

export const lectureMapAtom = derive([lecturesAtom], (lectures) => {
  return lectures
    ? new Map(lectures.map((lecture) => [lecture.id, lecture]))
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
    if (course && (!lecture || lectureMap?.has(lecture.id))) {
      set(internalLectureSelectAtom, (old) => {
        const map = new Map(old);
        map.set(uniqueKey(course), lecture);
        return map;
      });
    }
  },
);

export const lectureSelectIdAtom = atom(
  (get) => get(lectureSelectAtom)?.id ?? null,
  async (get, set, id: Lecture["id"] | null) => {
    const map = await get(lectureMapAtom);
    const lecture = id ? map?.get(id) : null;
    set(lectureSelectAtom, lecture ?? null);
  },
);
