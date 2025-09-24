import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { uniqueKey } from "@/features/course/services/pages";
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
        {[...completed].reverse().map((item) => (
          <ListItem key={uniqueKey(item)} item={item} status="completed" />
        ))}
      </div>
    </div>
  );
}
