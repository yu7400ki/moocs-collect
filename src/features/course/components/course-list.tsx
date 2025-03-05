import { useAtom, useAtomValue } from "jotai";
import { courseSelectIdAtom, coursesAtom } from "../atoms/course";
import { uniqueKey } from "../services/courses";
import * as ListItem from "./list-item";

export function CourseList() {
  const courses = useAtomValue(coursesAtom);
  const [selectedCourseId, setSelectedCourseId] = useAtom(courseSelectIdAtom);

  return (
    <ListItem.Root selected={selectedCourseId} onSelect={setSelectedCourseId}>
      {courses.map((course) => (
        <ListItem.Item key={uniqueKey(course)} value={course.id}>
          {course.name}
        </ListItem.Item>
      ))}
    </ListItem.Root>
  );
}
