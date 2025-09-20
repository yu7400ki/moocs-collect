import { load } from "@tauri-apps/plugin-store";
import { memoizeAsync } from "./cache";

export const getStore = memoizeAsync(() => load("store.json"));
