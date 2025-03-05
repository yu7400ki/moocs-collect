import { RadioGroup } from "@ark-ui/react";
import { css } from "styled-system/css";

export type RootProps<T extends string> = {
  children?: React.ReactNode;
  selected?: T | null;
  onSelect?: (value: T) => void;
} & Omit<
  React.ComponentProps<typeof RadioGroup.Root>,
  "value" | "onValueChange" | "onSelect"
>;

export function Root<T extends string>({
  children,
  selected,
  onSelect,
  ...props
}: RootProps<T>) {
  return (
    <RadioGroup.Root
      value={selected}
      onValueChange={(details) => onSelect?.(details.value as T)}
      {...props}
    >
      {children}
    </RadioGroup.Root>
  );
}

export type ItemProps<T extends string> = {
  value: T;
  children: React.ReactNode;
} & React.ComponentProps<typeof RadioGroup.Item>;

export function Item<T extends string>({
  value,
  children,
  ...props
}: ItemProps<T>) {
  return (
    <RadioGroup.Item
      value={value}
      {...props}
      className={css({
        appearance: "none",
        h: 8,
        px: 3,
        rounded: "l2",
        color: "fg.default",
        cursor: "pointer",
        display: "flex",
        alignItems: "center",
        outline: "none",
        position: "relative",
        transitionDuration: "normal",
        transitionProperty: "background, border-color, color, box-shadow",
        transitionTimingFunction: "default",
        userSelect: "none",
        verticalAlign: "middle",
        whiteSpace: "nowrap",
        _checked: {
          bg: "gray.a3",
          cursor: "default",
        },
      })}
    >
      <RadioGroup.ItemControl />
      <RadioGroup.ItemText
        className={css({
          flex: 1,
          overflow: "hidden",
          textOverflow: "ellipsis",
        })}
      >
        {children}
      </RadioGroup.ItemText>
      <RadioGroup.ItemHiddenInput />
    </RadioGroup.Item>
  );
}
