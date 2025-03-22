import { getSettings as getSettingsCommand } from "@/command/get-settings";
import { getStore } from "@/utils/store";
import * as path from "@tauri-apps/api/path";
import { type Settings, settingsSchema } from "../schemas/settings";

const key = "settings";

async function getDefaultSettings(): Promise<Settings> {
  const document = await path.documentDir();
  const downloadDir = await path.join(document, "moocs-collect");
  return {
    theme: "system",
    downloadDir,
  };
}

export async function getSettings() {
  const settings = await getSettingsCommand();
  const result = settingsSchema.safeParse(settings);
  if (result.success) {
    return result.data;
  }
  const store = await getStore();
  const defaultSettings = await getDefaultSettings();
  await store.set(key, defaultSettings);
  return defaultSettings;
}

export async function setSettings(settings: Settings) {
  const store = await getStore();
  const result = settingsSchema.safeParse(settings);
  if (result.success) {
    await store.set(key, result.data);
    return result.data;
  }
  return await getDefaultSettings();
}
