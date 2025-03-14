import { createCommand } from "./utils";

export type Args = {
  year?: number;
};

export type Output = {
  year: number;
  id: string;
  name: string;
}[];

export const getCourses = createCommand<Args, Output>("get_courses");
