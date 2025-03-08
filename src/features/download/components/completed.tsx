import { uniqueKey } from "@/features/course/services/pages";
import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { queueAtom } from "../atoms/queue";
import { ListItem } from "./list-item";

export function Completed() {
  const { completed } = useAtomValue(queueAtom);

  return (
    <div>
      <div className={css({ display: "grid" })}>
        {[...completed].reverse().map((page) => (
          <ListItem key={uniqueKey(page)} page={page} status="completed" />
        ))}
      </div>
    </div>
  );
}
