import { openPath } from "@tauri-apps/plugin-opener";
import { useSetAtom } from "jotai";
import { ExternalLinkIcon, RotateCcwIcon } from "lucide-react";
import { useCallback, useTransition } from "react";
import { css } from "styled-system/css";
import { IconButton } from "@/components/ui/icon-button";
import { Spinner } from "@/components/ui/spinner";
import { type DownloadItem, retryAtom } from "../atoms/queue";

export type Props =
  | {
      item: DownloadItem;
      status: "pending" | "running";
    }
  | {
      item: DownloadItem & { path?: string };
      status: "completed";
    }
  | {
      item: DownloadItem & { reason?: string };
      status: "error";
    };

export function ListItem({ item, status }: Props) {
  return (
    <div
      className={css({
        display: "grid",
        gridTemplateAreas: `
        "subtitle status"
        "title status"
        `,
        py: 2,
        px: 3,
        rowGap: 0.5,
        columnGap: 2,
        borderBottom: "1px solid",
        borderColor: "border.subtle",
      })}
    >
      <p
        className={css({
          gridArea: "subtitle",
          color: "fg.subtle",
          fontSize: "xs",
        })}
      >
        {item.course.name}
        <span className={css({ mx: 1 })}>/</span>
        {item.lecture.name}
      </p>
      <p
        className={css({
          gridArea: "title",
        })}
      >
        {item.name}
      </p>
      <div
        className={css({
          display: "grid",
          placeContent: "center",
          gridArea: "status",
          justifySelf: "end",
          alignSelf: "center",
          color: "fg.muted",
          fontSize: "sm",
        })}
      >
        <Status item={item} status={status} />
      </div>
    </div>
  );
}

function Status({ item, status }: Props) {
  switch (status) {
    case "pending": {
      return "待機中";
    }
    case "running": {
      return <Spinner size="sm" />;
    }
    case "completed": {
      return item.path ? <OpenButton path={item.path} /> : null;
    }
    case "error": {
      return <RetryButton item={item} />;
    }
    default: {
      return null;
    }
  }
}

function OpenButton({ path }: { path: string }) {
  const [isPending, startTransition] = useTransition();

  const handleOpenSlide = useCallback(() => {
    startTransition(() => {
      openPath(path);
    });
  }, [path]);

  return (
    <IconButton
      aria-label="開く"
      onClick={handleOpenSlide}
      disabled={isPending}
      variant="ghost"
      size="xs"
    >
      <ExternalLinkIcon />
    </IconButton>
  );
}

function RetryButton({ item }: { item: DownloadItem }) {
  const retry = useSetAtom(retryAtom);

  return (
    <IconButton
      aria-label="再試行"
      onClick={() => {
        retry(item);
      }}
      variant="ghost"
      size="xs"
    >
      <RotateCcwIcon />
    </IconButton>
  );
}
