import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { courseChecksAtom, toggleCourseCheckAtom } from "../atoms/check";
import { courseSelectIdAtom, coursesAtom } from "../atoms/course";
import { uniqueKey } from "../services/courses";
import { ListItem } from "./list-item";

export function CourseList() {
  const courses = useAtomValue(coursesAtom);
  const [selectedCourseId, setSelectedCourseId] = useAtom(courseSelectIdAtom);
  const courseChecks = useAtomValue(courseChecksAtom);
  const toggleChecks = useSetAtom(toggleCourseCheckAtom);

  return (
    <div>
      {courses.map((course) => (
        <ListItem
          key={uniqueKey(course)}
          value={course.id}
          selected={course.id === selectedCourseId}
          onSelect={setSelectedCourseId}
          checked={courseChecks.has(course.id)}
          onToggleCheck={toggleChecks}
        >
          {course.name}
        </ListItem>
      ))}
    </div>
  );
}
