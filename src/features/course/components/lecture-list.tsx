import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { Fragment, useMemo } from "react";
import { css } from "styled-system/css";
import { lectureChecksAtom, toggleLectureCheckAtom } from "../atoms/check";
import { lectureSelectIdAtom, lecturesAtom } from "../atoms/lecture";
import type { Lecture } from "../schemas/lecture";
import { uniqueKey } from "../services/lectures";
import { ListItem } from "./list-item";

function aggregateByGroup(lectures: Lecture[]) {
  return lectures.reduce(
    (acc, lecture) => {
      const group = lecture.group;
      if (!acc[group]) {
        acc[group] = [];
      }
      acc[group].push(lecture);
      return acc;
    },
    {} as Record<string, Lecture[]>,
  );
}

export function LectureList() {
  const lectures = useAtomValue(lecturesAtom);
  const [selectedLectureId, setSelectedLectureId] =
    useAtom(lectureSelectIdAtom);
  const lectureChecks = useAtomValue(lectureChecksAtom);
  const toggleChecks = useSetAtom(toggleLectureCheckAtom);

  const groupedLectures = useMemo(
    () => lectures && aggregateByGroup(lectures),
    [lectures],
  );

  return (
    <div>
      {Object.entries(groupedLectures || {}).map(([group, lectures]) => (
        <Fragment key={`${uniqueKey(lectures[0])}-${group}`}>
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
            {group}
          </span>
          {lectures.map((lecture) => (
            <ListItem
              key={uniqueKey(lecture)}
              value={lecture.id}
              selected={lecture.id === selectedLectureId}
              onSelect={setSelectedLectureId}
              checked={lectureChecks.has(lecture.id)}
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
