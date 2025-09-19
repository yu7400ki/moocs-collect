import { createTreeCollection } from "@ark-ui/react/tree-view";
import type { CheckedState } from "@zag-js/checkbox";
import { ChevronRightIcon } from "lucide-react";
import {
  type ComponentProps,
  type ReactNode,
  useCallback,
  useMemo,
  useState,
} from "react";
import { Checkbox } from "./checkbox";
import * as StyledTreeView from "./styled/tree-view";

const ROOT_NODE_ID = "__root__";

export type TreeSelectNode = {
  id: string;
  label: ReactNode;
  disabled?: boolean;
  children?: TreeSelectNode[];
};

type NodeStatus = {
  checked: boolean;
  indeterminate: boolean;
};

type RootProps = ComponentProps<typeof StyledTreeView.Root>;

function collectDescendantIds(node: TreeSelectNode): string[] {
  const stack: TreeSelectNode[] = [node];
  const ids: string[] = [];
  while (stack.length > 0) {
    const current = stack.pop();
    if (!current) {
      continue;
    }
    ids.push(current.id);
    if (current.children) {
      for (const child of current.children) {
        stack.push(child);
      }
    }
  }
  return ids;
}

export interface TreeSelectProps
  extends Omit<
    RootProps,
    | "collection"
    | "selectionMode"
    | "selectedValue"
    | "defaultSelectedValue"
    | "onSelectionChange"
  > {
  data: TreeSelectNode[];
  value?: string[];
  defaultValue?: string[];
  onValueChange?: (value: string[]) => void;
}

export function TreeSelect(props: TreeSelectProps) {
  const {
    data,
    value,
    defaultValue,
    onValueChange,
    defaultExpandedValue,
    expandedValue,
    ...rest
  } = props;

  const isControlled = value !== undefined;
  const [internalValue, setInternalValue] = useState<string[]>(
    defaultValue ?? [],
  );
  const selectedValues = isControlled ? (value ?? []) : internalValue;

  const { collection, nodeMap, parentMap, branchIds } = useMemo(() => {
    const nodeMap = new Map<string, TreeSelectNode>();
    const parentMap = new Map<string, string | null>();
    const branchIds: string[] = [];

    const visit = (node: TreeSelectNode, parentId: string | null) => {
      nodeMap.set(node.id, node);
      parentMap.set(node.id, parentId);
      if (node.children && node.children.length > 0) {
        branchIds.push(node.id);
        for (const child of node.children) {
          visit(child, node.id);
        }
      }
    };

    for (const node of data) {
      visit(node, null);
    }

    const rootNode: TreeSelectNode = {
      id: ROOT_NODE_ID,
      label: ROOT_NODE_ID,
      children: data,
    };

    const collection = createTreeCollection<TreeSelectNode>({
      rootNode,
      nodeToValue: (node) => node.id,
      nodeToString: (node) =>
        typeof node.label === "string" ? node.label : String(node.id),
      nodeToChildren: (node) => node.children ?? [],
      isNodeDisabled: (node) => Boolean(node.disabled),
    });

    return { collection, nodeMap, parentMap, branchIds };
  }, [data]);

  const isAncestorSelected = useCallback(
    (nodeId: string, selection: Set<string>) => {
      let currentId = parentMap.get(nodeId) ?? null;
      while (currentId) {
        if (selection.has(currentId)) {
          return true;
        }
        currentId = parentMap.get(currentId) ?? null;
      }
      return false;
    },
    [parentMap],
  );

  const compressSelection = useCallback(
    (selection: Set<string>) => {
      const compressed = new Set<string>();
      for (const id of selection) {
        if (id === ROOT_NODE_ID) {
          continue;
        }
        if (!isAncestorSelected(id, selection)) {
          compressed.add(id);
        }
      }
      return compressed;
    },
    [isAncestorSelected],
  );

  const normalizedSelectedValues = useMemo(() => {
    const compressed = compressSelection(new Set(selectedValues));
    return collection.sort(Array.from(compressed));
  }, [collection, compressSelection, selectedValues]);

  const selectedSet = useMemo(() => {
    return new Set(normalizedSelectedValues);
  }, [normalizedSelectedValues]);

  const commitSelection = useCallback(
    (selection: Set<string>) => {
      const compressed = compressSelection(selection);
      const nextValues = collection.sort(Array.from(compressed));
      if (!isControlled) {
        setInternalValue(nextValues);
      }
      onValueChange?.(nextValues);
    },
    [collection, compressSelection, isControlled, onValueChange],
  );

  const isFullyChecked = (
    node: TreeSelectNode,
    selection: Set<string>,
  ): boolean => {
    const children = node.children ?? [];
    if (children.length === 0) {
      return selection.has(node.id);
    }
    for (const child of children) {
      if (!isFullyChecked(child, selection)) {
        return false;
      }
    }
    return selection.has(node.id);
  };

  const toggleDescendants = (
    node: TreeSelectNode,
    checked: boolean,
    selection: Set<string>,
  ) => {
    const ids = collectDescendantIds(node);
    if (checked) {
      for (const id of ids) {
        if (!nodeMap.get(id)?.disabled) {
          selection.add(id);
        }
      }
    } else {
      for (const id of ids) {
        selection.delete(id);
      }
    }
  };

  const updateAncestors = (startId: string, selection: Set<string>) => {
    let currentId = parentMap.get(startId);
    while (currentId) {
      const parentNode = nodeMap.get(currentId);
      if (!parentNode) {
        break;
      }
      const allChildrenChecked =
        parentNode.children?.every((child) =>
          isFullyChecked(child, selection),
        ) ?? false;
      if (allChildrenChecked) {
        selection.add(currentId);
      } else {
        selection.delete(currentId);
      }
      currentId = parentMap.get(currentId) ?? null;
    }
  };

  const materializeSelectionFor = (nodeId: string, selection: Set<string>) => {
    let currentId = parentMap.get(nodeId);
    while (currentId) {
      if (selection.has(currentId)) {
        selection.delete(currentId);
        const ancestorNode = nodeMap.get(currentId);
        if (ancestorNode) {
          for (const id of collectDescendantIds(ancestorNode)) {
            selection.add(id);
          }
        }
      }
      currentId = parentMap.get(currentId) ?? null;
    }
  };

  const handleCheckboxChange = (nodeId: string, checked: boolean) => {
    const node = nodeMap.get(nodeId);
    if (!node) {
      return;
    }
    const selection = new Set(selectedSet);
    materializeSelectionFor(nodeId, selection);
    toggleDescendants(node, checked, selection);
    updateAncestors(nodeId, selection);
    commitSelection(selection);
  };

  const statusMap = useMemo(() => {
    const map = new Map<string, NodeStatus>();

    const computeStatus = (node: TreeSelectNode): NodeStatus => {
      const children = node.children ?? [];
      if (
        selectedSet.has(node.id) ||
        isAncestorSelected(node.id, selectedSet)
      ) {
        const status: NodeStatus = { checked: true, indeterminate: false };
        map.set(node.id, status);
        for (const child of children) {
          computeStatus(child);
        }
        return status;
      }

      if (children.length === 0) {
        const status: NodeStatus = { checked: false, indeterminate: false };
        map.set(node.id, status);
        return status;
      }

      const childStatuses = children.map((child) => computeStatus(child));
      const allChildrenChecked = childStatuses.every(
        (status) => status.checked,
      );
      const someChildrenChecked = childStatuses.some(
        (status) => status.checked || status.indeterminate,
      );
      const status: NodeStatus = {
        checked: allChildrenChecked,
        indeterminate: !allChildrenChecked && someChildrenChecked,
      };
      map.set(node.id, status);
      return status;
    };

    for (const node of data) {
      computeStatus(node);
    }
    return map;
  }, [data, isAncestorSelected, selectedSet]);

  return (
    <StyledTreeView.Root
      {...rest}
      collection={collection}
      selectionMode="multiple"
      expandedValue={expandedValue}
      defaultExpandedValue={
        expandedValue !== undefined
          ? undefined
          : (defaultExpandedValue ?? branchIds)
      }
    >
      <StyledTreeView.Tree>
        {collection.rootNode.children?.map((node, index) => (
          <TreeNodeView
            key={node.id}
            node={node}
            indexPath={[index]}
            statusMap={statusMap}
            onToggle={handleCheckboxChange}
          />
        ))}
      </StyledTreeView.Tree>
    </StyledTreeView.Root>
  );
}

interface TreeNodeViewProps {
  node: TreeSelectNode;
  indexPath: number[];
  statusMap: Map<string, NodeStatus>;
  onToggle: (id: string, checked: boolean) => void;
}

function TreeNodeView(props: TreeNodeViewProps) {
  const { node, indexPath, statusMap, onToggle } = props;
  const status = statusMap.get(node.id) ?? {
    checked: false,
    indeterminate: false,
  };

  const handleChange = (checked: CheckedState) => {
    const nextChecked = checked === true || checked === "indeterminate";
    onToggle(node.id, nextChecked);
  };

  const checkbox = (
    <Checkbox
      checked={getCheckedState(status)}
      disabled={node.disabled}
      onCheckedChange={({ checked }) => handleChange(checked)}
      onClick={(event) => event.stopPropagation()}
      onPointerDown={(event) => event.stopPropagation()}
      size="sm"
    />
  );

  if (node.children && node.children.length > 0) {
    return (
      <StyledTreeView.NodeProvider node={node} indexPath={indexPath}>
        <StyledTreeView.Branch>
          <StyledTreeView.BranchControl>
            <StyledTreeView.BranchText>
              {checkbox}
              {node.label}
            </StyledTreeView.BranchText>
            <StyledTreeView.BranchIndicator>
              <ChevronRightIcon />
            </StyledTreeView.BranchIndicator>
          </StyledTreeView.BranchControl>
          <StyledTreeView.BranchContent>
            <StyledTreeView.BranchIndentGuide />
            {node.children.map((child, childIndex) => (
              <TreeNodeView
                key={child.id}
                node={child}
                indexPath={[...indexPath, childIndex]}
                statusMap={statusMap}
                onToggle={onToggle}
              />
            ))}
          </StyledTreeView.BranchContent>
        </StyledTreeView.Branch>
      </StyledTreeView.NodeProvider>
    );
  }

  return (
    <StyledTreeView.NodeProvider node={node} indexPath={indexPath}>
      <StyledTreeView.Item>
        <StyledTreeView.ItemText>
          {checkbox}
          {node.label}
        </StyledTreeView.ItemText>
      </StyledTreeView.Item>
    </StyledTreeView.NodeProvider>
  );
}

function getCheckedState(status: NodeStatus): CheckedState {
  if (status.indeterminate) {
    return "indeterminate";
  }
  return status.checked;
}
