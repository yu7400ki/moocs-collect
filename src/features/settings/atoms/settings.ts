import { memoizeAsync } from "@/utils/cache";
import * as path from "@tauri-apps/api/path";
import { load } from "@tauri-apps/plugin-store";
import { atom } from "jotai";
import { type Settings, settingsSchema } from "../schemas/settings";

type Update<T> = T | ((prev: T) => T);

const key = "settings";

const getStore = memoizeAsync(() => load("store.json"));

async function getDefaultSettings() {
  const document = await path.documentDir();
  const downloadDir = await path.join(document, "moocs-collect");
  return {
    theme: "system",
    downloadDir,
  } satisfies Settings;
}

const internalSettingsAtom = atom<Settings | null>(null);
internalSettingsAtom.onMount = (setSettings) => {
  (async () => {
    const store = await getStore();
    const settings = await store.get<Settings>(key);
    const defaultSettings = await getDefaultSettings();
    const result = settingsSchema.safeParse({
      ...defaultSettings,
      ...settings,
    });
    if (result.success) {
      setSettings(result.data);
      await store.set(key, result.data);
    }
  })();
};

export const settingsAtom = atom(
  (get) => get(internalSettingsAtom),
  async (get, set, update: Update<Settings | null>) => {
    const store = await getStore();
    const settings =
      typeof update === "function" ? update(get(settingsAtom)) : update;
    const result = settingsSchema.safeParse(settings);
    if (result.success) {
      await store.set(key, result.data);
      set(internalSettingsAtom, result.data);
    } else {
      set(internalSettingsAtom, await getDefaultSettings());
    }
  },
);
