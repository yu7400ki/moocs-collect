import { useAtomValue } from "jotai";
import { css } from "styled-system/css";
import { uniqueKey } from "@/features/course/services/pages";
import { queueAtom } from "../atoms/queue";
import { Empty } from "./empty";
import { ListItem } from "./list-item";

export function Errors() {
  const { error } = useAtomValue(queueAtom);

  if (error.size === 0) {
    return <Empty />;
  }

  return (
    <div>
      <div className={css({ display: "grid" })}>
        {[...error].reverse().map((item) => (
          <ListItem key={uniqueKey(item)} item={item} status="error" />
        ))}
      </div>
    </div>
  );
}
