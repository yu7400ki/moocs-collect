import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseId: string;
  lectureId: string;
  pageId: string;
};

export type Output = {
  content: string[];
}[];

export const getSlides = createCommand<Args, Output>("get_pages");
