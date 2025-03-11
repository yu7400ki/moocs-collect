import { atom } from "jotai";
import { settingsAtom } from "./settings";

export const yearAtom = atom((get) => get(settingsAtom)?.year);
