import { PackageOpenIcon } from "lucide-react";
import { css } from "styled-system/css";

export function Empty() {
  return (
    <div
      className={css({
        display: "grid",
        placeItems: "center",
        placeContent: "center",
        gap: 4,
        h: "full",
        color: "fg.muted",
      })}
    >
      <p
        className={css({
          fontSize: "sm",
        })}
      >
        ここにはまだ何もありません
      </p>
      <PackageOpenIcon size={42} />
    </div>
  );
}
