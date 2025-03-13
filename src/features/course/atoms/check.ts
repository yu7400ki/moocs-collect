import type { MaybePromise } from "@/utils/types";
import { type Atom, type WritableAtom, atom } from "jotai";
import { soon } from "jotai-derive";
import { courseSelectIdAtom, coursesAtom } from "./course";
import { lectureSelectIdAtom, lecturesAtom } from "./lecture";
import { pagesAtom } from "./page";

export type Node = {
  id: string;
  checked: boolean;
  children?: Node[];
};

function createTreeAtom(
  initialNode: Node,
  childrenDataAtom: Atom<{ id: string }[] | Promise<{ id: string }[]> | null>,
) {
  const internalAtom = atom(initialNode);
  return atom(
    (get) => {
      return soon(get(childrenDataAtom), (data) => {
        const tree = get(internalAtom);
        const set = new Set(data?.map(({ id }) => id));
        const children = tree.children?.filter((node) => set.has(node.id));
        return {
          ...tree,
          children: children ?? [],
        };
      });
    },
    (_, set, children: Node[]) => {
      set(internalAtom, (prev) => {
        const newTree = structuredClone(prev);
        newTree.children = children;
        return newTree;
      });
    },
  );
}

function createTreeOperationAtom<R>(
  treeAtom: WritableAtom<MaybePromise<Node | null>, [Node[]], R>,
) {
  return {
    addAtom: atom(null, async (get, set, child: Node) => {
      const tree = await get(treeAtom);
      const newNodes =
        structuredClone(tree)?.children?.filter(
          (node) => node.id !== child.id,
        ) ?? [];
      newNodes.push({ ...child, checked: true });
      set(treeAtom, newNodes);
    }),
    deleteAtom: atom(null, async (get, set, childId: string) => {
      const tree = await get(treeAtom);
      const newNodes =
        structuredClone(tree)?.children?.filter(
          (node) => node.id !== childId,
        ) ?? [];
      set(treeAtom, newNodes);
    }),
  };
}

function createChildTreeAtom<R>(
  parentTreeAtom: WritableAtom<MaybePromise<Node | null>, [Node[]], R>,
  selectedParentIdAtom: Atom<string | null>,
  childrenDataAtom: Atom<{ id: string }[] | Promise<{ id: string }[]> | null>,
  parentOperationAtom: ReturnType<typeof createTreeOperationAtom>,
) {
  const childAtom = atom(
    (get) => {
      const selectedParentId = get(selectedParentIdAtom);
      if (!selectedParentId) {
        return null;
      }
      return soon(get(parentTreeAtom), (parentTree) => {
        const node =
          parentTree?.children?.find((node) => node.id === selectedParentId) ??
          ({
            id: selectedParentId,
            checked: false,
            children: [],
          } satisfies Node);
        const copiedNode = structuredClone(node);
        if (!copiedNode?.children) {
          return soon(get(childrenDataAtom), (data) => {
            const children = (data ?? []).reduce((nodes, child) => {
              nodes.push({ id: child.id, checked: true });
              return nodes;
            }, [] as Node[]);
            copiedNode.children = children;
            return copiedNode;
          });
        }
        return copiedNode;
      });
    },
    async (get, set, children: Node[]) => {
      const tree = await get(childAtom);
      if (!tree) {
        return;
      }
      if (children.length === 0) {
        set(parentOperationAtom.deleteAtom, tree.id);
        return;
      }
      tree.checked = true;
      tree.children = children;
      set(parentOperationAtom.addAtom, tree);
    },
  );
  return childAtom;
}

export const courseTreeAtom = createTreeAtom(
  {
    id: "root",
    checked: true,
    children: [],
  },
  coursesAtom,
);

export const lectureTreeAtom = createChildTreeAtom(
  courseTreeAtom,
  courseSelectIdAtom,
  lecturesAtom,
  createTreeOperationAtom(courseTreeAtom),
);

export const pageTreeAtom = createChildTreeAtom(
  lectureTreeAtom,
  lectureSelectIdAtom,
  pagesAtom,
  createTreeOperationAtom(lectureTreeAtom),
);

function createChecksAtom<R>(
  treeAtom: WritableAtom<MaybePromise<Node | null>, [Node[]], R>,
) {
  return atom(
    (get) => {
      return soon(get(treeAtom), (tree) => {
        return new Set<string>(
          tree?.children
            ?.filter((node) => node.checked)
            .map((node) => node.id) ?? [],
        );
      });
    },
    async (get, set, ids: string[]) => {
      const tree = await get(treeAtom);
      const nodes = ids.map((id) => ({
        ...tree?.children?.find((node) => node.id === id),
        id,
        checked: true,
      }));
      set(treeAtom, nodes);
    },
  );
}

function createToggleAtom(checksAtom: ReturnType<typeof createChecksAtom>) {
  return atom(null, async (get, set, id: string) => {
    const checks = await get(checksAtom);
    const newChecks = new Set(checks);
    if (newChecks.has(id)) {
      newChecks.delete(id);
    } else {
      newChecks.add(id);
    }
    set(checksAtom, Array.from(newChecks));
  });
}

export const courseChecksAtom = createChecksAtom(courseTreeAtom);
export const toggleCourseCheckAtom = createToggleAtom(courseChecksAtom);

export const lectureChecksAtom = createChecksAtom(lectureTreeAtom);
export const toggleLectureCheckAtom = createToggleAtom(lectureChecksAtom);

export const pageChecksAtom = createChecksAtom(pageTreeAtom);
export const togglePageCheckAtom = createToggleAtom(pageChecksAtom);
