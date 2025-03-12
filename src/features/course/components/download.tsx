import { store } from "@/components/providers/jotai";
import { Button } from "@/components/ui/button";
import { getSettings } from "@/features/settings/services/settings";
import type { MaybePromise } from "@/utils/types";
import { DownloadIcon } from "lucide-react";
import { useCallback } from "react";
import { useTransition } from "react";
import { type Node, courseTreeAtom } from "../atoms/check";
import type { Page } from "../schemas/page";
import { getCourses } from "../services/courses";
import { getLectures } from "../services/lectures";
import { getPages } from "../services/pages";
import { queueAtom } from "@/features/download/atoms/queue";

async function delayedPromise<T>(value: MaybePromise<T>, ms = 1000) {
  if (value instanceof Promise) {
    await new Promise((resolve) => setTimeout(resolve, ms));
  }
  return await value;
}

function intoNode<T extends { id: string }>(data: T): Node {
  return {
    id: data.id,
    checked: true,
  };
}

function nonNullable<T>(v: T): v is NonNullable<T> {
  return v !== null && v !== undefined;
}

function getCheckedPairs<T extends { id: string }>(
  nodes: Node[] | undefined,
  items: T[],
  defaultMap: (item: T) => Node = intoNode,
): [T, Node][] {
  const nodeList = nodes ?? items.map(defaultMap);
  return nodeList
    .filter((node) => node.checked)
    .map((node) => {
      const item = items.find((item) => item.id === node.id);
      return item ? ([item, node] as [T, Node]) : null;
    })
    .filter(nonNullable);
}

function enqueuePages(pages: Page[]) {
  for (const page of pages) {
    store.set(queueAtom, page);
  }
}

async function retrievePages(node: Node) {
  const settings = await getSettings();
  const courses = await delayedPromise(getCourses(settings));
  const coursePairs = getCheckedPairs(node.children, courses);

  for (const [course, courseNode] of coursePairs) {
    const lectures = await delayedPromise(getLectures(course));
    const lecturePairs = getCheckedPairs(courseNode.children, lectures);

    for (const [lecture, lectureNode] of lecturePairs) {
      const pages = await delayedPromise(getPages(lecture));
      const pagePairs = getCheckedPairs(lectureNode.children, pages);
      enqueuePages(pagePairs.map(([page]) => page));
    }
  }
}

export function Download({
  onClick,
  ...props
}: React.ComponentProps<typeof Button>) {
  const [isPending, startTransition] = useTransition();

  const handleClick = useCallback(
    async (e: React.MouseEvent<HTMLButtonElement>) => {
      onClick?.(e);
      const tree = await store.get(courseTreeAtom);
      await retrievePages(tree);
      store.set(courseTreeAtom, []);
    },
    [onClick],
  );

  return (
    <Button
      onClick={(e) => {
        startTransition(async () => {
          await handleClick(e);
        });
      }}
      size="sm"
      variant="subtle"
      loading={isPending}
      {...props}
    >
      ダウンロード
      <DownloadIcon />
    </Button>
  );
}
