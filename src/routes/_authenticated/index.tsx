import { CoursePage } from "@/features/course/pages/course";
import { createFileRoute } from "@tanstack/react-router";

export const Route = createFileRoute("/_authenticated/")({
  component: CoursePage,
});
