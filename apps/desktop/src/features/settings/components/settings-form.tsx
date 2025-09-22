import { useAtom } from "jotai";
import { XIcon } from "lucide-react";
import { useCallback, useState, useTransition } from "react";
import { css } from "styled-system/css";
import { Divider, Stack } from "styled-system/jsx";
import { Button } from "@/components/ui/button";
import { Dialog } from "@/components/ui/dialog";
import { IconButton } from "@/components/ui/icon-button";
import { Toast } from "@/components/ui/toast";
import { settingsAtom } from "../atoms/settings";
import { purgeIndex } from "../services/purge-index";
import { DirSelector } from "./dir-selector";
import { ThemeSelector } from "./theme-selector";

function event(f: (name: string) => (value: string) => void) {
  return (e: React.ChangeEvent<HTMLInputElement>) => {
    f(e.target.name)(e.target.value);
  };
}

const toaster = Toast.createToaster({
  placement: "bottom-end",
  overlap: true,
  gap: 16,
  duration: 5000,
});

export function SettingsForm() {
  const [settings, setSettings] = useAtom(settingsAtom);
  const [isPending, startTransition] = useTransition();
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  const handleChange = useCallback(
    (name: string) => (value: string) => {
      setSettings((prev) => {
        if (!prev) {
          return prev;
        }
        return {
          ...prev,
          [name]: value,
        };
      });
    },
    [setSettings],
  );

  const handlePurgeIndex = useCallback(() => {
    startTransition(async () => {
      try {
        await purgeIndex();
        toaster.create({
          title: "検索インデックスを初期化しました",
          type: "success",
        });
      } catch (e) {
        const message =
          e instanceof Error
            ? e.message
            : typeof e === "string"
              ? e
              : "不明なエラー";
        toaster.create({
          title: "エラーが発生しました",
          description: message,
          type: "error",
        });
        return;
      } finally {
        setIsDialogOpen(false);
      }
    });
  }, []);

  return (
    <>
      <form className={css({ display: "grid", gap: 6, maxW: "2xl" })}>
        <div className={css({ display: "grid", gap: 1.5 })}>
          <p>テーマ</p>
          <ThemeSelector
            name="theme"
            value={settings?.theme}
            onChange={event(handleChange)}
          />
        </div>
        <Divider />
        <div className={css({ display: "grid", gap: 1.5 })}>
          <p>ダウンロード先</p>
          <DirSelector
            name="downloadDir"
            value={settings?.downloadDir}
            onChange={handleChange("downloadDir")}
          />
        </div>
        <Divider />
        <div
          className={css({
            display: "flex",
            alignItems: "center",
            justifyContent: "space-between",
          })}
        >
          <p>検索インデックスを初期化する</p>
          <Button
            type="button"
            colorPalette="red"
            onClick={() => setIsDialogOpen(true)}
          >
            初期化
          </Button>
        </div>
      </form>
      <Dialog.Root
        open={isDialogOpen}
        onOpenChange={(details) => setIsDialogOpen(details.open)}
      >
        <Dialog.Backdrop />
        <Dialog.Positioner>
          <Dialog.Content minW="0" maxW="90vw">
            <Stack gap="8" p="6">
              <Stack gap="1">
                <Dialog.Title>検索インデックスを初期化する</Dialog.Title>
                <Dialog.Description>
                  この操作は元に戻せません。
                  <br />
                  再びインデックスを作成するには、スライドの再ダウンロードが必要です。
                </Dialog.Description>
              </Stack>
              <Stack
                gap="3"
                direction="row"
                justifyContent="flex-end"
                flexWrap="wrap"
              >
                <Dialog.CloseTrigger asChild>
                  <Button variant="outline">キャンセル</Button>
                </Dialog.CloseTrigger>
                <Button
                  colorPalette="red"
                  onClick={handlePurgeIndex}
                  loading={isPending}
                >
                  初期化
                </Button>
              </Stack>
            </Stack>
            <Dialog.CloseTrigger asChild position="absolute" top="2" right="2">
              <IconButton aria-label="閉じる" variant="ghost" size="sm">
                <XIcon />
              </IconButton>
            </Dialog.CloseTrigger>
          </Dialog.Content>
        </Dialog.Positioner>
      </Dialog.Root>
      <Toast.Toaster toaster={toaster}>
        {(toast) => (
          <Toast.Root key={toast.id} className="group">
            <Toast.Title
              css={{
                ".group[data-type='error'] &": {
                  color: "fg.error",
                },
              }}
            >
              {toast.title}
            </Toast.Title>
            <Toast.Description>{toast.description}</Toast.Description>
            <Toast.CloseTrigger asChild>
              <IconButton size="sm" variant="link">
                <XIcon />
              </IconButton>
            </Toast.CloseTrigger>
          </Toast.Root>
        )}
      </Toast.Toaster>
    </>
  );
}
