import { uniqueKey } from "@/features/course/services/pages";
import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { queueAtom } from "../atoms/queue";
import { ListItem } from "./list-item";

export function InQueue() {
  const { running, pending } = useAtomValue(queueAtom);

  return (
    <div>
      <div className={css({ display: "grid" })}>
        {[...running].map((page) => (
          <ListItem key={uniqueKey(page)} page={page} status="running" />
        ))}
        {[...pending].map((page) => (
          <ListItem key={uniqueKey(page)} page={page} status="pending" />
        ))}
      </div>
    </div>
  );
}
