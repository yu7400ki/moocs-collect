import { createCommand } from "./utils";

export type Args = {
  params: {
    year: number;
    courseSlug: string;
    lectureSlug: string;
    pageSlug: string;
  };
};

export type Output = string | undefined;

export const downloadSlides = createCommand<Args, Output>("download_slides");
