import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { Fragment } from "react";
import { css } from "styled-system/css";
import { lectureChecksAtom, toggleLectureCheckAtom } from "../atoms/check";
import { lectureGroupsAtom, lectureSelectSlugAtom } from "../atoms/lecture";
import { uniqueKey } from "../services/lectures";
import { ListItem } from "./list-item";

export function LectureList() {
  const lectureGroups = useAtomValue(lectureGroupsAtom);
  const [selectedLectureSlug, setSelectedLectureSlug] = useAtom(
    lectureSelectSlugAtom,
  );
  const lectureChecks = useAtomValue(lectureChecksAtom);
  const toggleChecks = useSetAtom(toggleLectureCheckAtom);

  return (
    <div>
      {lectureGroups?.map((group) => (
        <Fragment key={`${group.year}-${group.courseSlug}-${group.name}`}>
          <span
            className={css({
              display: "block",
              fontSize: "sm",
              color: "fg.muted",
              pl: 3,
              "&:not(:first-child)": {
                mt: 2,
              },
            })}
          >
            {group.name}
          </span>
          {group.lectures.map((lecture) => (
            <ListItem
              key={uniqueKey(lecture)}
              value={lecture.slug}
              selected={lecture.slug === selectedLectureSlug}
              onSelect={setSelectedLectureSlug}
              checked={lectureChecks.has(lecture.slug)}
              onToggleCheck={toggleChecks}
            >
              {lecture.name}
            </ListItem>
          ))}
        </Fragment>
      ))}
    </div>
  );
}
