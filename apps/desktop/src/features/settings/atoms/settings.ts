import { atom } from "jotai";
import type { Settings } from "../schemas/settings";
import { getSettings, setSettings } from "../services/settings";

type Update<T> = T | ((prev: T) => T);

const internalSettingsAtom = atom<Settings | null>(null);
internalSettingsAtom.onMount = (setSettings) => {
  (async () => {
    const settings = await getSettings();
    setSettings(settings);
  })();
};

export const settingsAtom = atom(
  (get) => get(internalSettingsAtom),
  async (get, set, update: Update<Settings | null>) => {
    const settings =
      typeof update === "function" ? update(get(settingsAtom)) : update;
    if (settings) {
      const result = await setSettings(settings);
      set(internalSettingsAtom, result);
    }
  },
);
