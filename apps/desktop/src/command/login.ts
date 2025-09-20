import { createCommand } from "./utils";

export type Args = {
  username: string;
  password: string;
  remember: boolean;
};

export type Output = boolean;

export const login = createCommand<Args, Output>("login");
