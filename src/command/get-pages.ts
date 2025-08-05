import { createCommand } from "./utils";

export type Args = {
  year: number;
  courseSlug: string;
  lectureSlug: string;
};

export type Output = {
  year: number;
  courseSlug: string;
  lectureSlug: string;
  slug: string;
  name: string;
}[];

export const getPages = createCommand<Args, Output>("get_pages");
