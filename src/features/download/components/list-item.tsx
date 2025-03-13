import { IconButton } from "@/components/ui/icon-button";
import { Spinner } from "@/components/ui/spinner";
import type { Page } from "@/features/course/schemas/page";
import { useSetAtom } from "jotai";
import { RotateCcwIcon } from "lucide-react";
import { css } from "styled-system/css";
import { retryAtom } from "../atoms/queue";

export type Props = {
  page: Page;
  status: "pending" | "running" | "completed" | "error";
};

export function ListItem({ page, status }: Props) {
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
        {page.lecture.course.name}
        <span className={css({ mx: 1 })}>/</span>
        {page.lecture.name}
      </p>
      <p
        className={css({
          gridArea: "title",
        })}
      >
        {page.title}
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
        <Status page={page} status={status} />
      </div>
    </div>
  );
}

function Status({ page, status }: Props) {
  switch (status) {
    case "pending": {
      return "待機中";
    }
    case "running": {
      return <Spinner size="sm" />;
    }
    case "completed": {
      return null;
    }
    case "error": {
      return <RetryButton page={page} />;
    }
    default: {
      return null;
    }
  }
}

function RetryButton({ page }: { page: Page }) {
  const retry = useSetAtom(retryAtom);

  return (
    <IconButton
      aria-label="再試行"
      onClick={() => {
        retry(page);
      }}
      variant="ghost"
      size="xs"
    >
      <RotateCcwIcon />
    </IconButton>
  );
}
