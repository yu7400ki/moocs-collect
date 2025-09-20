import { open } from "@tauri-apps/plugin-dialog";
import { useControllableValue } from "ahooks";
import { useCallback, useTransition } from "react";
import { css } from "styled-system/css";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

type DirSelectorProps = Omit<React.ComponentProps<typeof Input>, "onChange"> & {
  value?: string;
  defaultValue?: string;
  onChange?: (value: string) => void;
};

export function DirSelector({
  value,
  defaultValue,
  onChange,
  ...props
}: DirSelectorProps) {
  const [dir, setDir] = useControllableValue<string>({
    value,
    defaultValue,
    onChange,
  });
  const [isPending, startTransition] = useTransition();

  const handleSelectDir = useCallback(async () => {
    const result = await open({
      directory: true,
      multiple: false,
      defaultPath: dir,
    });

    if (result) {
      setDir(result);
    }
  }, [dir, setDir]);

  return (
    <div
      className={css({
        display: "grid",
        gridTemplateColumns: "1fr auto",
        gap: 2,
      })}
    >
      <Input value={dir} {...props} readOnly size="sm" />
      <Button
        size="sm"
        onClick={() => startTransition(async () => await handleSelectDir())}
        disabled={isPending}
      >
        変更する...
      </Button>
    </div>
  );
}
