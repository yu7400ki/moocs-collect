import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseId: string;
  lectureId: string;
  pageId: string;
};

export type Output = undefined;

export const downloadSlides = createCommand<Args, Output>("download_slides");
