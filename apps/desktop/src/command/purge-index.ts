import { createCommand } from "./utils";

export type Args = undefined;

export type Output = undefined;

export const purgeIndex = createCommand<Args, Output>("purge_index");
