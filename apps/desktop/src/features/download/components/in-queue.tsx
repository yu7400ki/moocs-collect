import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { uniqueKey } from "@/features/course/services/pages";
import { queueAtom } from "../atoms/queue";
import { Empty } from "./empty";
import { ListItem } from "./list-item";

export function InQueue() {
  const { running, pending } = useAtomValue(queueAtom);

  if (running.size === 0 && pending.size === 0) {
    return <Empty />;
  }

  return (
    <div>
      <div className={css({ display: "grid" })}>
        {[...running].map((item) => (
          <ListItem key={uniqueKey(item)} item={item} status="running" />
        ))}
        {[...pending].map((item) => (
          <ListItem key={uniqueKey(item)} item={item} status="pending" />
        ))}
      </div>
    </div>
  );
}
