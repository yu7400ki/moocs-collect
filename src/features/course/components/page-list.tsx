import { useAtom, useAtomValue } from "jotai";
import { pageSelectIdAtom, pagesAtom } from "../atoms/page";
import { uniqueKey } from "../services/pages";
import * as ListItem from "./list-item";

export function PageList() {
  const pages = useAtomValue(pagesAtom);
  const [selectedPageId, setSelectedPageId] = useAtom(pageSelectIdAtom);

  return (
    <ListItem.Root selected={selectedPageId} onSelect={setSelectedPageId}>
      {pages?.map((page) => (
        <ListItem.Item key={uniqueKey(page)} value={page.id}>
          {page.title}
        </ListItem.Item>
      ))}
    </ListItem.Root>
  );
}
