import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseId: string;
  lectureId: string;
};

export type Output = {
  year: number;
  courseId: string;
  lectureId: string;
  id: string;
  title: string;
}[];

export const getPages = createCommand<Args, Output>("get_pages");
