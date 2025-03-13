import { css } from "styled-system/css";
import { Divider } from "styled-system/jsx";
import { Column } from "../components/column";
import { Download } from "../components/download";

export function CoursePage() {
  return (
    <main
      className={css({
        display: "grid",
        gridTemplateRows: "auto auto minmax(0, 1fr)",
      })}
    >
      <Download justifySelf="end" m="1" />
      <Divider orientation="horizontal" />
      <Column />
    </main>
  );
}
