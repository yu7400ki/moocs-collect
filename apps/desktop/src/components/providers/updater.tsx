import { Button } from "@/components/ui/button";
import { IconButton } from "@/components/ui/icon-button";
import { Progress } from "@/components/ui/progress";
import { Toast } from "@/components/ui/toast";
import { relaunch } from "@tauri-apps/plugin-process";
import { type Update, check } from "@tauri-apps/plugin-updater";
import { XIcon } from "lucide-react";
import { useCallback, useEffect, useRef, useState } from "react";
import { css } from "styled-system/css";

const toaster = Toast.createToaster({
  placement: "bottom-end",
  duration: Number.POSITIVE_INFINITY,
});

export function Updater() {
  const mountedRef = useRef(false);
  const updateRef = useRef<Update | null>(null);
  const toastIdRef = useRef<string | null>(null);
  const [contentLength, setContentLength] = useState(0);
  const [downloaded, setDownloaded] = useState(0);

  const handleInstall = useCallback(async () => {
    if (!updateRef.current) {
      return;
    }
    const update = updateRef.current;
    await update.install();
    await relaunch();
  }, []);

  const handleUpdate = useCallback(async () => {
    if (!updateRef.current) {
      return;
    }
    const update = updateRef.current;
    const toastId = toastIdRef.current;
    if (toastId) {
      toaster.update(toastId, {
        description: "ダウンロード中...",
        type: "loading",
      });
    }
    await update.download((event) => {
      switch (event.event) {
        case "Started": {
          setContentLength(event.data.contentLength ?? 0);
          break;
        }
        case "Progress": {
          setDownloaded((prev) => prev + event.data.chunkLength);
          break;
        }
        case "Finished": {
          if (toastId) {
            toaster.update(toastId, {
              description: "ダウンロード完了",
              type: "success",
            });
          }
          break;
        }
        default: {
          break;
        }
      }
    });
  }, []);

  useEffect(() => {
    if (mountedRef.current) {
      return;
    }
    mountedRef.current = true;
    (async () => {
      const update = await check();
      updateRef.current = update;
      if (update) {
        toastIdRef.current = toaster.create({
          title: "更新",
          description: "更新が利用可能です。",
          type: "info",
        });
      }
    })();
  }, []);

  return (
    <Toast.Toaster toaster={toaster}>
      {(toast) => (
        <Toast.Root
          key={toast.id}
          className={css({
            display: "grid",
            gridTemplateAreas: `
            "title action"
            "description action"
          `,
            alignItems: "center",
          })}
        >
          <Toast.Title gridArea="title">{toast.title}</Toast.Title>
          <Toast.Description gridArea="description">
            {toast.description}
          </Toast.Description>
          {toast.type === "info" && (
            <Button
              variant="link"
              size="sm"
              onClick={handleUpdate}
              gridArea="action"
              justifySelf="end"
              mx="2"
            >
              更新
            </Button>
          )}
          {toast.type === "loading" && (
            <Progress
              value={downloaded}
              max={contentLength}
              type="circular"
              size="xs"
              gridArea="action"
              width="fit-content"
              justifySelf="end"
              mx="2"
            />
          )}
          {toast.type === "success" && (
            <Button
              variant="link"
              size="sm"
              onClick={handleInstall}
              gridArea="action"
              justifySelf="end"
              mx="2"
            >
              再起動
            </Button>
          )}
          {toast.type === "info" && (
            <Toast.CloseTrigger asChild top="1.5" right="1.5">
              <IconButton size="sm" variant="link">
                <XIcon />
              </IconButton>
            </Toast.CloseTrigger>
          )}
        </Toast.Root>
      )}
    </Toast.Toaster>
  );
}
