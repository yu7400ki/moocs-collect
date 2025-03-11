import { NumberInput } from "@/components/ui/number-input";
import { useAtom } from "jotai";
import { useCallback } from "react";
import { css } from "styled-system/css";
import { Divider } from "styled-system/jsx";
import { settingsAtom } from "../atoms/settings";
import { DirSelector } from "./dir-selector";
import { ThemeSelector } from "./theme-selector";

function event(f: (name: string) => (value: string) => void) {
  return (e: React.ChangeEvent<HTMLInputElement>) => {
    f(e.target.name)(e.target.value);
  };
}

export function SettingsForm() {
  const [settings, setSettings] = useAtom(settingsAtom);

  const handleChange = useCallback(
    (name: string) => (value: string) => {
      setSettings((prev) => {
        if (!prev) {
          return prev;
        }
        return {
          ...prev,
          [name]: value,
        };
      });
    },
    [setSettings],
  );

  return (
    <form className={css({ display: "grid", gap: 6, maxW: "2xl" })}>
      <div className={css({ display: "grid", gap: 1.5 })}>
        <p>テーマ</p>
        <ThemeSelector
          name="theme"
          value={settings?.theme}
          onChange={event(handleChange)}
        />
      </div>
      <Divider />
      <div className={css({ display: "grid", gap: 1.5 })}>
        <p>ダウンロード先</p>
        <DirSelector
          name="downloadDir"
          value={settings?.downloadDir}
          onChange={handleChange("downloadDir")}
        />
      </div>
      <Divider />
      <div className={css({ display: "grid", gap: 1.5 })}>
        <p>年度</p>
        <NumberInput
          name="year"
          value={String(settings?.year ?? "")}
          onValueChange={({ value }) => handleChange("year")(value)}
          max={new Date().getFullYear()}
          height="9"
          maxW="40"
        />
      </div>
    </form>
  );
}
