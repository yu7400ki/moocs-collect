import { createFileRoute } from "@tanstack/react-router";
import { CoursePage } from "@/features/course/pages/course";

export const Route = createFileRoute("/_authenticated/")({
  component: CoursePage,
});
