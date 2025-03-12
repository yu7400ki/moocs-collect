import { uniqueKey } from "@/features/course/services/pages";
import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { queueAtom } from "../atoms/queue";
import { Empty } from "./empty";
import { ListItem } from "./list-item";

export function Completed() {
  const { completed } = useAtomValue(queueAtom);

  if (completed.size === 0) {
    return <Empty />;
  }

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
