import { Spinner } from "@/components/ui/spinner";
import type { Page } from "@/features/course/schemas/page";
import { css } from "styled-system/css";

export type Props = {
  page: Page;
  status: "pending" | "running" | "completed";
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
          gridArea: "status",
          justifySelf: "end",
          alignSelf: "center",
          color: "fg.muted",
          fontSize: "sm",
        })}
      >
        <Status status={status} />
      </div>
    </div>
  );
}

function Status({ status }: Pick<Props, "status">) {
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
    default: {
      return null;
    }
  }
}
