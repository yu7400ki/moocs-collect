import { RadioGroup } from "@/components/ui/radio-group";
import { MonitorIcon, MoonIcon, SunIcon } from "lucide-react";
import { css, cx } from "styled-system/css";
import { Divider } from "styled-system/jsx";

type ThemeCardProps = {
  value: string;
  label: string;
  className?: string;
  children?: React.ReactNode;
};

function ThemeCard({ value, label, className, children }: ThemeCardProps) {
  return (
    <RadioGroup.Item
      value={value}
      className={css({
        outline: "1px solid",
        outlineColor: "border.default",
        display: "grid",
        rounded: "l2",
        gap: 0,
        overflow: "hidden",
        _checked: {
          outline: "2px solid",
          outlineColor: "colorPalette.default",
        },
      })}
    >
      <div
        className={cx(
          className,
          css({
            h: 24,
            w: 40,
            display: "grid",
            placeContent: "center",
          }),
        )}
      >
        {children}
      </div>
      <Divider />
      <div
        className={css({
          display: "flex",
          alignItems: "center",
          gap: 2,
          px: 2,
          py: 1,
        })}
      >
        <RadioGroup.ItemControl />
        <RadioGroup.ItemText>{label}</RadioGroup.ItemText>
      </div>
      <RadioGroup.ItemHiddenInput />
    </RadioGroup.Item>
  );
}

export function ThemeSelector(
  props: React.ComponentProps<typeof RadioGroup.Root>,
) {
  return (
    <RadioGroup.Root
      size="sm"
      orientation="horizontal"
      className={css({
        display: "flex",
        flexWrap: "wrap",
      })}
      {...props}
    >
      <ThemeCard
        value="system"
        label="自動"
        className={css({
          _osLight: {
            bg: "gray.light.1",
            color: "gray.light.12",
            stroke: "gray.light.12",
          },
          _osDark: {
            bg: "gray.dark.1",
            color: "gray.dark.12",
            stroke: "gray.dark.12",
          },
        })}
      >
        <MonitorIcon size={28} />
      </ThemeCard>
      <ThemeCard
        value="light"
        label="ライト"
        className={css({
          bg: "gray.light.1",
          color: "gray.light.12",
          stroke: "gray.light.12",
        })}
      >
        <SunIcon size={28} />
      </ThemeCard>
      <ThemeCard
        value="dark"
        label="ダーク"
        className={css({
          bg: "gray.dark.1",
          color: "gray.dark.12",
          stroke: "gray.dark.12",
        })}
      >
        <MoonIcon size={28} />
      </ThemeCard>
    </RadioGroup.Root>
  );
}
