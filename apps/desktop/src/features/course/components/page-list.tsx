import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { pageChecksAtom, togglePageCheckAtom } from "../atoms/check";
import { pageSelectSlugAtom, pagesAtom } from "../atoms/page";
import { uniqueKey } from "../services/pages";
import { ListItem } from "./list-item";

export function PageList() {
  const pages = useAtomValue(pagesAtom);
  const [selectedPageSlug, setSelectedPageSlug] = useAtom(pageSelectSlugAtom);
  const pageChecks = useAtomValue(pageChecksAtom);
  const setPageChecks = useSetAtom(togglePageCheckAtom);

  return (
    <div>
      {pages?.map((page) => (
        <ListItem
          key={uniqueKey(page)}
          value={page.slug}
          selected={page.slug === selectedPageSlug}
          onSelect={setSelectedPageSlug}
          checked={pageChecks.has(page.slug)}
          onToggleCheck={setPageChecks}
        >
          {page.name}
        </ListItem>
      ))}
    </div>
  );
}
