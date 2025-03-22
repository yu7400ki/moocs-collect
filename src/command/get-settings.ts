import { createCommand } from "./utils";

export type Args = undefined;

export type Output = {
  theme: "system" | "light" | "dark";
  downloadDir: string;
  year?: number;
};

export const getSettings = createCommand<Args, Output>("get_settings");
