import { css } from "styled-system/css";
import { Divider, Flex } from "styled-system/jsx";
import { Column } from "../components/column";
import { Download } from "../components/download";
import { YearSelect } from "../components/year-select";

export function CoursePage() {
  return (
    <main
      className={css({
        display: "grid",
        gridTemplateRows: "auto auto minmax(0, 1fr)",
      })}
    >
      <Flex m="1" justifyContent="space-between">
        <YearSelect />
        <Download />
      </Flex>
      <Divider orientation="horizontal" />
      <Column />
    </main>
  );
}
