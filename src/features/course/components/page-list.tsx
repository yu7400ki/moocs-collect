import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { pageChecksAtom, togglePageCheckAtom } from "../atoms/check";
import { pageSelectIdAtom, pagesAtom } from "../atoms/page";
import { uniqueKey } from "../services/pages";
import { ListItem } from "./list-item";

export function PageList() {
  const pages = useAtomValue(pagesAtom);
  const [selectedPageId, setSelectedPageId] = useAtom(pageSelectIdAtom);
  const pageChecks = useAtomValue(pageChecksAtom);
  const setPageChecks = useSetAtom(togglePageCheckAtom);

  return (
    <div>
      {pages?.map((page) => (
        <ListItem
          key={uniqueKey(page)}
          value={page.id}
          selected={page.id === selectedPageId}
          onSelect={setSelectedPageId}
          checked={pageChecks.has(page.id)}
          onToggleCheck={setPageChecks}
        >
          {page.title}
        </ListItem>
      ))}
    </div>
  );
}
