import { createCommand } from "./utils";

export type Args = {
  username: string;
};

export type Output = string | undefined;

export const getCredential = createCommand<Args, Output>("get_credential");
