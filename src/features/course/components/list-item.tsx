import { Checkbox } from "@/components/ui/checkbox";
import { memo } from "react";
import { css } from "styled-system/css";

export type ListItemProps<T extends string> = {
  children?: React.ReactNode;
  value: T;
  selected?: boolean;
  onSelect?: (value: NoInfer<T>) => void;
  checked?: boolean;
  onToggleCheck?: (value: NoInfer<T>) => void;
};

function ListItemComponent<T extends string>({
  children,
  value,
  selected,
  onSelect,
  checked,
  onToggleCheck,
}: ListItemProps<T>) {
  return (
    <button
      type="button"
      className={css({
        appearance: "none",
        h: 8,
        w: "full",
        px: 3,
        rounded: "l2",
        color: "fg.default",
        cursor: "pointer",
        display: "flex",
        alignItems: "center",
        outline: "none",
        position: "relative",
        textAlign: "left",
        transitionDuration: "normal",
        transitionProperty: "background, border-color, color, box-shadow",
        transitionTimingFunction: "default",
        userSelect: "none",
        verticalAlign: "middle",
        whiteSpace: "nowrap",
        _checked: {
          bg: "gray.a3",
        },
      })}
      data-state={selected ? "checked" : undefined}
      onClick={() => onSelect?.(value)}
      onDoubleClick={() => onToggleCheck?.(value)}
    >
      <Checkbox
        onCheckedChange={() => onToggleCheck?.(value)}
        onDoubleClick={(e) => e.stopPropagation()}
        checked={checked}
        className={css({
          mr: 1.5,
        })}
      />
      <span
        className={css({
          flex: 1,
          overflow: "hidden",
          textOverflow: "ellipsis",
        })}
      >
        {children}
      </span>
    </button>
  );
}

export const ListItem = memo(ListItemComponent) as typeof ListItemComponent;
