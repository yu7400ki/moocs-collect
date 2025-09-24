import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseSlug: string;
};

export type Lecture = {
  year: number;
  courseSlug: string;
  slug: string;
  name: string;
  index: number;
};

export type Output = {
  year: number;
  courseSlug: string;
  name: string;
  lectures: Lecture[];
  index: number;
}[];

export const getLectures = createCommand<Args, Output>("get_lectures");
