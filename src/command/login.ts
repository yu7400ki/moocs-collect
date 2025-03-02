import { createCommand } from "./utils";

export type Args = {
  username: string;
  password: string;
};

export type Output = boolean;

export const login = createCommand<Args, Output>("login");
