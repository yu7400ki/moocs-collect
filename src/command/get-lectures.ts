import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseId: string;
};

export type Output = {
  year: number;
  courseId: string;
  id: string;
  name: string;
  group: string;
}[];

export const getLectures = createCommand<Args, Output>("get_lectures");
