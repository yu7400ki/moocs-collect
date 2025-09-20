import { createCommand } from "./utils";

export type Args = undefined;

export type Output = number[];

export const getArchiveYears = createCommand<Args, Output>("get_archive_years");
