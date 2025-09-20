import { Spinner } from "@/components/ui/spinner";
import { Fragment, Suspense } from "react";
import { css } from "styled-system/css";
import { Divider } from "styled-system/jsx";
import { CourseList } from "./course-list";
import { LectureList } from "./lecture-list";
import { PageList } from "./page-list";

function Loading() {
  return (
    <div
      className={css({
        position: "absolute",
        inset: 0,
        display: "grid",
        placeItems: "center",
      })}
    >
      <Spinner size="md" />
    </div>
  );
}

function Section({ children }: { children: React.ReactNode }) {
  return (
    <div
      className={css({
        overflowY: "auto",
        position: "relative",
        p: 2,
      })}
    >
      <Suspense fallback={<Loading />}>{children}</Suspense>
    </div>
  );
}

const components = [CourseList, LectureList, PageList];

export function Column() {
  return (
    <div
      className={css({
        display: "grid",
        gridTemplateColumns: "1fr auto 1fr auto 1fr",
      })}
    >
      {components.map((Component, index) => (
        // biome-ignore lint/suspicious/noArrayIndexKey: This is a static list
        <Fragment key={index}>
          <Section>
            <Component />
          </Section>
          {index < components.length - 1 && <Divider orientation="vertical" />}
        </Fragment>
      ))}
    </div>
  );
}
