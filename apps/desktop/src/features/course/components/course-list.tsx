import { useAtom, useAtomValue, useSetAtom } from "jotai";
import { courseChecksAtom, toggleCourseCheckAtom } from "../atoms/check";
import { courseSelectSlugAtom, coursesAtom } from "../atoms/course";
import { uniqueKey } from "../services/courses";
import { ListItem } from "./list-item";

export function CourseList() {
  const courses = useAtomValue(coursesAtom);
  const [selectedCourseSlug, setSelectedCourseSlug] =
    useAtom(courseSelectSlugAtom);
  const courseChecks = useAtomValue(courseChecksAtom);
  const toggleChecks = useSetAtom(toggleCourseCheckAtom);

  return (
    <div>
      {courses.map((course) => (
        <ListItem
          key={uniqueKey(course)}
          value={course.slug}
          selected={course.slug === selectedCourseSlug}
          onSelect={setSelectedCourseSlug}
          checked={courseChecks.has(course.slug)}
          onToggleCheck={toggleChecks}
        >
          {course.name}
        </ListItem>
      ))}
    </div>
  );
}
