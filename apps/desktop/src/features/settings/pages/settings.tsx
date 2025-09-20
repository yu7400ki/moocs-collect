import { css } from "styled-system/css";
import { SettingsForm } from "../components/settings-form";

export function SettingsPage() {
  return (
    <main className={css({ p: 4, minH: "fit-content" })}>
      <h1
        className={css({
          fontSize: "xl",
          mb: 4,
        })}
      >
        設定
      </h1>
      <SettingsForm />
    </main>
  );
}
