import { createCommand } from "./utils";

export type Args = undefined;

export type RecordedCourse = {
  year: number;
  slug: string;
  name: string;
  sortIndex: number;
};

export type Output = RecordedCourse[];

export const getRecordedCourses = createCommand<Args, Output>(
  "get_recorded_courses",
);
